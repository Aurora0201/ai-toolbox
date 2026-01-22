<template>
  <div class="flex flex-col h-full overflow-hidden bg-background-app relative">
    <!-- Messages Container -->
    <div
      ref="messagesRef"
      class="flex-1 overflow-y-auto pt-4 pb-60 scroll-smooth"
    >
      <div class="max-w-3xl mx-auto w-full min-h-full flex flex-col">
        <div
          v-if="translatorStore.messages.length === 0"
          class="flex-1 flex flex-col items-center justify-center opacity-40 select-none"
        >
          <div class="text-6xl mb-4 animate-bounce">
            🌐
          </div>
          <h2 class="text-xl font-black uppercase tracking-[0.2em] text-text-sub">
            {{ $t('translator.startTranslation') }}
          </h2>
          <p class="text-sm mt-2 font-mono">
            {{ $t('chat.noHistoryWarning') }}
          </p>
        </div>

        <template v-else>
          <ChatMessage
            v-for="(msg, index) in translatorStore.messages"
            :key="index"
            v-bind="msg"
            :is-generating="translatorStore.isGenerating && index === translatorStore.messages.length - 1"
          />
        </template>
      </div>
    </div>

    <!-- Bottom Gradient Mask & Input Area -->
    <div class="absolute bottom-0 left-0 right-0 h-60 bg-gradient-to-t from-background-app to-transparent pointer-events-none z-10" />
    
    <div class="absolute bottom-0 left-0 right-0 p-6 pointer-events-none z-20">
      <div class="w-full flex justify-center pointer-events-auto">
        <TranslatorInput
          v-model:selected-model="modelStore.selectedModel"
          v-model:target-language="translatorStore.targetLanguage"
          :running-models="modelStore.runningModels"
          :is-generating="translatorStore.isGenerating"
          :messages-count="translatorStore.messages.length"
          :disabled="!modelStore.selectedModel"
          :placeholder="modelStore.selectedModel ? $t('translator.placeholder') : $t('translator.selectToStart')"
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
import { useTranslatorStore } from '../store/translator'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import ChatMessage from '../components/chat/ChatMessage.vue'
import TranslatorInput from '../components/translator/TranslatorInput.vue'
import { useI18n } from 'vue-i18n'

const modelStore = useModelStore()
const settingsStore = useSettingsStore()
const translatorStore = useTranslatorStore()
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

const handleSend = async ({ text }) => {
  if (translatorStore.isGenerating) return
  
  // Show user's original text
  translatorStore.addMessage('user', text)
  scrollToBottom(true)
  
  translatorStore.setGenerating(true)
  translatorStore.addMessage('assistant', '', '', false, modelStore.selectedModel) // Add empty assistant message for streaming
  
  let accumulatedContent = ''
  let accumulatedThinking = ''
  isProcessing = true
  
  try {
    // Listen for streaming events
    unlistenFn = await listen('chat-response', async (event) => {
      if (!isProcessing) return // Ignore if stopped
      
      const payload = event.payload
      
      if (payload.done) {
        if (payload.content) accumulatedContent += payload.content
        if (payload.thinking) accumulatedThinking += payload.thinking
        translatorStore.updateLastMessage(accumulatedContent, accumulatedThinking, false)

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
        translatorStore.setGenerating(false)
        if (unlistenFn) {
            unlistenFn()
            unlistenFn = null
        }
        return
      }
      
      if (payload.content) accumulatedContent += payload.content
      if (payload.thinking) accumulatedThinking += payload.thinking
      
      translatorStore.updateLastMessage(accumulatedContent, accumulatedThinking, payload.is_thinking)
      scrollToBottom()
    })

    // Prepare generation options
    const options = { ...settingsStore.generationParameters }
    if (options.num_predict === -1) options.num_predict = undefined
    if (options.seed === -1) options.seed = undefined

    // Construct Translation Prompt
    // Note: This is a simple prompt strategy. For better results, one might use system prompts if the model supports it.
    // Here we wrap the request in a clear instruction.
    const prompt = `Translate the following text into ${translatorStore.targetLanguage}. Do not provide any explanations, notes, or introductions. Just provide the translated text.

Text to translate:
${text}`

    // Start generation via Backend Command
    await invoke('generate_completion', {
      request: {
        model: modelStore.selectedModel,
        prompt: prompt,
        images: [],
        options: options,
        stream: true
      }
    })

  } catch (error) {
    console.error('Translation failed:', error)
    translatorStore.updateLastMessage(t('chat.errorMessage') + error)
    translatorStore.setGenerating(false)
    if (unlistenFn) {
        unlistenFn()
        unlistenFn = null
    }
  }
}

const handleStop = async () => {
  isProcessing = false
  translatorStore.setGenerating(false)
  if (unlistenFn) {
      unlistenFn()
      unlistenFn = null
  }
  
  translatorStore.updateLastMessage(translatorStore.messages[translatorStore.messages.length - 1].content + `\n\n*[${t('chat.stopped')}]*`, translatorStore.messages[translatorStore.messages.length - 1].thinking)
}

const handleClear = () => {
  translatorStore.clearMessages()
}

// Watch for messages change to scroll
watch(() => translatorStore.messages.length, () => {
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
.flex-1 {
  scrollbar-gutter: stable;
}
</style>