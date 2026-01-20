<template>
  <div class="flex flex-col h-full p-6 max-w-[900px] mx-auto">
    <div
      ref="messagesRef"
      class="flex-1 overflow-y-auto mb-6 pr-2"
    >
      <div
        v-for="(msg, index) in messages"
        :key="index"
        class="mb-6"
      >
        <div
          class="flex gap-3 max-w-[85%]"
          :class="msg.role === 'user' ? 'ml-auto justify-end' : ''"
        >
          <div
            v-if="msg.role === 'assistant'"
            class="w-9 h-9 bg-background-surface border border-border rounded-full flex items-center justify-center shrink-0 shadow-sm"
          >
            <Bot class="w-5 h-5 text-primary" />
          </div>
          
          <div
            class="flex flex-col"
            :class="msg.role === 'user' ? 'items-end' : ''"
          >
            <div class="mb-1">
              <span
                class="text-[10px] font-bold text-text-sub uppercase font-mono"
              >{{ msg.role === 'user' ? $t('chat.you') : $t('chat.ai') }}</span>
            </div>
            <div 
              class="p-3 border rounded-lg shadow-sm"
              :class="msg.role === 'user' ? 'bg-primary/5 border-primary/20 rounded-tr-none' : 'bg-background-surface border-border rounded-tl-none'"
            >
              <div class="text-sm leading-relaxed whitespace-pre-wrap text-text-main">
                {{ msg.content }}
              </div>
            </div>
          </div>

          <div
            v-if="msg.role === 'user'"
            class="w-9 h-9 bg-background-surface border border-border rounded-full flex items-center justify-center shrink-0 shadow-sm"
          >
            <User class="w-5 h-5 text-text-sub" />
          </div>
        </div>
      </div>
      
      <div
        v-if="loading"
        class="mb-6"
      >
        <div class="flex gap-3 max-w-[85%]">
          <div class="w-9 h-9 bg-background-surface border border-border rounded-full flex items-center justify-center shrink-0 shadow-sm">
            <Bot class="w-5 h-5 text-primary" />
          </div>
          <div class="flex flex-col">
            <div class="mb-1">
              <span class="text-[10px] font-bold text-text-sub uppercase font-mono">{{ $t('chat.ai') }}</span>
            </div>
            <div class="p-3 bg-background-surface border border-border rounded-lg shadow-sm rounded-tl-none">
              <div class="text-muted flex items-center gap-2 text-sm">
                <Loader2 class="w-4 h-4 animate-spin" /> {{ $t('chat.thinking') }}
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>

    <div class="bg-background-surface border border-primary/50 rounded-lg shadow-md">
      <div class="p-2 px-3">
        <select
          v-model="store.selectedModel"
          class="w-auto bg-transparent border-none font-semibold text-text-main text-sm focus:ring-0 cursor-pointer outline-none py-1"
          @change="store.selectModel($event.target.value)"
        >
          <option
            disabled
            value=""
          >
            {{ $t('chat.selectModel') }}
          </option>
          <option
            v-for="model in store.models"
            :key="model.name"
            :value="model.name"
          >
            {{ model.name }}
          </option>
        </select>
      </div>
      <div class="p-3 pt-0">
        <div class="relative flex items-end">
          <textarea 
            v-model="input" 
            :placeholder="$t('chat.placeholder')" 
            :disabled="loading"
            rows="3"
            class="w-full border-none bg-transparent resize-none p-0 pr-10 focus:ring-0 text-sm outline-none text-text-main placeholder:text-text-sub"
            @keydown.enter.prevent="sendMessage"
          />
          <button
            :disabled="loading || !store.selectedModel"
            class="absolute bottom-0 right-0 p-2 text-primary hover:bg-background-element rounded-md disabled:text-text-sub transition-colors flex items-center justify-center cursor-pointer disabled:cursor-not-allowed"
            @click="sendMessage"
          >
            <SendHorizontal class="w-5 h-5" />
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
/**
 * Chat view component for interacting with models.
 */
import { ref, onMounted, nextTick, computed } from 'vue'
import { useModelStore } from '../store/models'
import { useSettingsStore } from '../store/settings'
import { invoke } from '@tauri-apps/api/core'
import { Bot, User, SendHorizontal, Loader2 } from 'lucide-vue-next'
import { useI18n } from 'vue-i18n'

const store = useModelStore()
const settings = useSettingsStore()
const { t } = useI18n()
const input = ref('')
const messages = ref([])
const loading = ref(false)
const messagesRef = ref(null)

/**
 * Sends a message to the selected Ollama model.
 */
const sendMessage = async () => {
  if (!input.value.trim() || !store.selectedModel) return

  const userMsg = input.value
  messages.value.push({ role: 'user', content: userMsg })
  input.value = ''
  loading.value = true
  
  await scrollToBottom()

  try {
    const endpoint = settings.ollamaEndpoint.endsWith('/') 
      ? `${settings.ollamaEndpoint}api/generate` 
      : `${settings.ollamaEndpoint}/api/generate`

    const response = await fetch(endpoint, {
      method: 'POST',
      body: JSON.stringify({
        model: store.selectedModel,
        prompt: userMsg,
        stream: false
      })
    })
    
    const data = await response.json()
    messages.value.push({ role: 'assistant', content: data.response })
    
    const today = new Date().toISOString().split('T')[0]
    await invoke('record_tokens', {
      date: today,
      prompt: data.prompt_eval_count || 0,
      completion: data.eval_count || 0,
      model: store.selectedModel
    })
    
  } catch (error) {
    messages.value.push({ role: 'assistant', content: t('chat.errorMessage') + error })
  } finally {
    loading.value = false
    await scrollToBottom()
  }
}

/**
 * Scrolls the message container to the bottom.
 */
const scrollToBottom = async () => {
  await nextTick()
  if (messagesRef.value) {
    messagesRef.value.scrollTop = messagesRef.value.scrollHeight
  }
}

onMounted(async () => {
  await store.fetchModels()
  if (!store.selectedModel && store.models.length > 0) {
    store.selectModel(store.models[0].name)
  }
})
</script>
