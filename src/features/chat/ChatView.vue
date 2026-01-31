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
import { useModelStore } from '../../store/models'
import { useChatStore } from '../../store/chat'
import ChatMessage from './components/ChatMessage.vue'
import ChatInput from './components/ChatInput.vue'
import BackgroundEffect from '../../shared/components/BackgroundEffect.vue'
import EmptyState from '../../shared/components/EmptyState.vue'
import { MessageSquareDashed } from 'lucide-vue-next'

const modelStore = useModelStore()
const chatStore = useChatStore()

const messagesRef = ref(null)

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
  await chatStore.sendMessage({
      text,
      images,
      model: modelStore.selectedModel
  })
}

const handleStop = async () => {
  await chatStore.stopGeneration()
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
  chatStore.stopGeneration() // Ensure stopped when leaving view
})
</script>

<style scoped>
/* Ensure smooth transitions for layout */
.flex-1 {
  scrollbar-gutter: stable;
}
</style>