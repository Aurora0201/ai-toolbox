<template>
  <div class="flex flex-col h-full overflow-hidden bg-background-app relative">
    <BackgroundEffect />
    <!-- Messages Container -->
    <div
      ref="messagesRef"
      class="flex-1 overflow-y-auto pt-4 pb-60 scroll-smooth"
    >
      <div class="max-w-3xl mx-auto w-full min-h-full flex flex-col">
        <div
          v-if="chatStore.messages.length === 0"
          class="flex-1 flex flex-col"
        >
          <EmptyState
            :icon="MessageSquareDashed"
            :title="$t('chat.startConversation')"
            :description="$t('chat.noHistoryWarning')"
          />
        </div>

        <template v-else>
          <ChatMessage
            v-for="(msg, index) in chatStore.messages"
            :key="index"
            v-bind="msg"
            :is-generating="chatStore.isGenerating && index === chatStore.messages.length - 1"
          />
        </template>
      </div>
    </div>

    <!-- Bottom Gradient Mask & Input Area -->
    <div class="absolute bottom-0 left-0 right-0 h-60 bg-gradient-to-t from-background-app to-transparent pointer-events-none z-10" />
    
    <div class="absolute bottom-0 left-0 right-0 p-6 pointer-events-none z-20">
      <div class="w-full flex justify-center pointer-events-auto">
        <ChatInput
          v-model:selected-model="modelStore.selectedModel"
          :running-models="modelStore.runningModels"
          :is-generating="chatStore.isGenerating"
          :messages-count="chatStore.messages.length"
          :disabled="!modelStore.selectedModel"
          :placeholder="modelStore.selectedModel ? $t('chat.placeholder') : $t('chat.selectToStart')"
          @send="handleSend"
          @stop="handleStop"
          @clear="handleClear"
        />
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted, nextTick, watch, onUnmounted } from 'vue'
import { useModelStore } from '../store/models'
import { useSettingsStore } from '../store/settings'
import { useChatStore } from '../store/chat'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import ChatMessage from '../components/chat/ChatMessage.vue'
import ChatInput from '../components/chat/ChatInput.vue'
import BackgroundEffect from '../components/common/BackgroundEffect.vue'
import EmptyState from '../components/common/EmptyState.vue'
import { MessageSquareDashed } from 'lucide-vue-next'
import { useI18n } from 'vue-i18n'

const modelStore = useModelStore()
const settingsStore = useSettingsStore()
const chatStore = useChatStore()
const { t } = useI18n()

const messagesRef = ref(null)
let unlistenFn = null
let isProcessing = false

const scrollToBottom = async (force = false) => {
  await nextTick()
  if (messagesRef.value) {
    const { scrollTop, scrollHeight, clientHeight } = messagesRef.value
    // Only scroll if we are already near bottom or force is true
    if (force || scrollHeight - scrollTop - clientHeight < 200) {
      messagesRef.value.scrollTop = messagesRef.value.scrollHeight
    }
  }
}

const handleSend = async ({ text, images }) => {
  if (chatStore.isGenerating) return
  
  chatStore.addMessage('user', text)
  scrollToBottom(true)
  
  chatStore.setGenerating(true)
  chatStore.addMessage('assistant', '', '', false, modelStore.selectedModel) // Add empty assistant message for streaming
  
  let accumulatedContent = ''
  let accumulatedThinking = ''
  isProcessing = true
  
  try {
    // Listen for streaming events
    unlistenFn = await listen('chat-response', async (event) => {
      if (!isProcessing) return // Ignore if stopped
      
      const payload = event.payload
      
      if (payload.done) {
        // Final update to ensure content/thinking and status (is_thinking: false) are captured
        if (payload.content) accumulatedContent += payload.content
        if (payload.thinking) accumulatedThinking += payload.thinking
        chatStore.updateLastMessage(accumulatedContent, accumulatedThinking, false)

        // Record usage
        const today = new Date().toISOString().split('T')[0]
        const promptEvalCount = payload.prompt_eval_count || 0
        const evalCount = payload.eval_count || 0
        
        await invoke('record_tokens', {
          date: today,
          prompt: promptEvalCount,
          completion: evalCount,
          model: modelStore.selectedModel
        })
        
        isProcessing = false
        chatStore.setGenerating(false)
        if (unlistenFn) {
            unlistenFn()
            unlistenFn = null
        }
        return
      }
      
      // Append new chunks from backend
      if (payload.content) accumulatedContent += payload.content
      if (payload.thinking) accumulatedThinking += payload.thinking
      
      chatStore.updateLastMessage(accumulatedContent, accumulatedThinking, payload.is_thinking)
      scrollToBottom()
    })

    // Prepare generation options
    const options = { ...settingsStore.generationParameters }
    // Clean up special values (-1 means default/random)
    if (options.num_predict === -1) options.num_predict = undefined
    if (options.seed === -1) options.seed = undefined

    // Start generation via Backend Command
    await invoke('generate_completion', {
      request: {
        model: modelStore.selectedModel,
        prompt: text,
        images: images.map(img => img.split(',')[1]), // Strip data:image/xxx;base64,
        options: options,
        stream: true
      }
    })

  } catch (error) {
    console.error('Generation failed:', error)
    chatStore.updateLastMessage(t('chat.errorMessage') + error)
    chatStore.setGenerating(false)
    if (unlistenFn) {
        unlistenFn()
        unlistenFn = null
    }
  }
}

const handleStop = async () => {
  isProcessing = false
  chatStore.setGenerating(false)
  if (unlistenFn) {
      unlistenFn()
      unlistenFn = null
  }
  // Optional: Call backend to kill process if needed, 
  // but just stopping the listener is enough for UI responsiveness
  // Real cancellation would require a backend abort handle which is complex with current structure
  
  chatStore.updateLastMessage(chatStore.messages[chatStore.messages.length - 1].content + `\n\n*[${t('chat.stopped')}]*`, chatStore.messages[chatStore.messages.length - 1].thinking)
}

const handleClear = () => {
  chatStore.clearMessages()
}

// Watch for messages change to scroll
watch(() => chatStore.messages.length, () => {
  scrollToBottom(true)
})

// Watch running models to validate selection and auto-select
watch(() => modelStore.runningModels, (newModels) => {
  // 1. If we have a selection, verify it still exists
  if (modelStore.selectedModel) {
    const exists = newModels.find(m => m.name === modelStore.selectedModel)
    if (!exists) {
      modelStore.selectModel('')
    }
  }
  
  // 2. If no selection (or just cleared), try to auto-select first available
  if (!modelStore.selectedModel && newModels.length > 0) {
    modelStore.selectModel(newModels[0].name)
  }
}, { deep: true })

onMounted(async () => {
  modelStore.startMonitoring()
  await modelStore.fetchRunningModels()
  if (modelStore.runningModels.length > 0 && !modelStore.selectedModel) {
    modelStore.selectModel(modelStore.runningModels[0].name)
  }
})

onUnmounted(() => {
  modelStore.stopMonitoring()
  if (unlistenFn) unlistenFn()
})
</script>

<style scoped>
/* Ensure smooth transitions for layout */
.flex-1 {
  scrollbar-gutter: stable;
}
</style>