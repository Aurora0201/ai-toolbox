<template>
  <div class="chat-view-container">
    <div
      ref="messagesRef"
      class="messages"
    >
      <div
        v-for="(msg, index) in messages"
        :key="index"
        :class="['message-wrapper', msg.role]"
      >
        <div class="message-row">
          <div
            v-if="msg.role === 'assistant'"
            class="avatar"
          >
            🤖
          </div>
          
          <div class="content-column">
            <div class="message-meta">
              <span
                class="role-badge text-mono"
                :class="msg.role"
              >{{ msg.role === 'user' ? t.you : t.ai }}</span>
            </div>
            <div class="message-bubble panel">
              <div class="panel-body">
                {{ msg.content }}
              </div>
            </div>
          </div>

          <div
            v-if="msg.role === 'user'"
            class="avatar"
          >
            👤
          </div>
        </div>
      </div>
      
      <div
        v-if="loading"
        class="message-wrapper assistant"
      >
        <div class="message-row">
          <div class="avatar">
            🤖
          </div>
          <div class="content-column">
            <div class="message-meta">
              <span class="role-badge assistant text-mono">{{ t.ai }}</span>
            </div>
            <div class="message-bubble panel">
              <div class="panel-body text-muted">
                <span class="typing-indicator">{{ t.thinking }} 💭</span>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>

    <div class="input-area panel">
      <div
        class="panel-header"
        style="background: transparent; padding: 8px 12px; border-bottom: none;"
      >
        <select
          v-model="store.selectedModel"
          class="model-select"
          @change="store.selectModel($event.target.value)"
        >
          <option
            disabled
            value=""
          >
            {{ t.selectModel }}
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
      <div
        class="panel-body"
        style="padding: 0 12px 12px 12px;"
      >
        <div class="input-wrapper">
          <textarea 
            v-model="input" 
            :placeholder="t.placeholder" 
            :disabled="loading"
            rows="3"
            @keydown.enter.prevent="sendMessage"
          />
          <button
            :disabled="loading || !store.selectedModel"
            class="btn btn-primary send-btn"
            @click="sendMessage"
          >
            <svg
              xmlns="http://www.w3.org/2000/svg"
              width="16"
              height="16"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              stroke-width="2"
              stroke-linecap="round"
              stroke-linejoin="round"
            ><line
              x1="22"
              y1="2"
              x2="11"
              y2="13"
            /><polygon points="22 2 15 22 11 13 2 9 22 2" /></svg>
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

const store = useModelStore()
const settings = useSettingsStore()
const input = ref('')
const messages = ref([])
const loading = ref(false)
const messagesRef = ref(null)

const translations = {
  en: {
    you: 'YOU',
    ai: 'AI',
    thinking: 'Thinking...',
    selectModel: 'Select Model',
    placeholder: 'Type your message here...',
    errorMessage: 'Error: '
  },
  zh: {
    you: '您',
    ai: 'AI',
    thinking: '正在思考...',
    selectModel: '选择模型',
    placeholder: '输入消息...',
    errorMessage: '错误: '
  }
}

const t = computed(() => translations[settings.language] || translations.en)

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
    messages.value.push({ role: 'assistant', content: t.value.errorMessage + error })
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

<style scoped>
.chat-view-container {
  display: flex;
  flex-direction: column;
  height: 100%;
  padding: 24px;
  max-width: 900px;
  margin: 0 auto;
}

.messages {
  flex: 1;
  overflow-y: auto;
  margin-bottom: 24px;
  padding-right: 8px;
}

.message-wrapper {
  margin-bottom: 24px;
}

.message-row {
  display: flex;
  gap: 12px;
  max-width: 80%;
}

.message-wrapper.user .message-row {
  flex-direction: row;
  margin-left: auto;
  justify-content: flex-end;
}

.avatar {
  width: 36px;
  height: 36px;
  background-color: white;
  border: 1px solid var(--border-color);
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 20px;
  flex-shrink: 0;
  box-shadow: var(--shadow-sm);
}

.content-column {
  display: flex;
  flex-direction: column;
}

.message-wrapper.user .content-column {
  align-items: flex-end;
}

.message-meta {
  margin-bottom: 4px;
}

.role-badge {
  font-size: 10px;
  font-weight: 700;
  color: var(--text-secondary);
  text-transform: uppercase;
}

.message-bubble {
  background: white;
  box-shadow: var(--shadow-sm);
  border-radius: var(--radius-md);
  position: relative;
}

.message-wrapper.assistant .message-bubble {
  border-top-left-radius: 0;
}

.message-wrapper.user .message-bubble {
  background: #f1f8ff;
  border-color: #cce5ff;
  border-top-right-radius: 0;
}

.input-area {
  flex-shrink: 0;
  border-color: var(--primary-color);
  box-shadow: var(--shadow-md);
}

.input-wrapper {
  position: relative;
  display: flex;
  align-items: flex-end;
}

textarea {
  width: 100%;
  border: none;
  background: transparent;
  resize: none;
  font-family: var(--font-sans);
  padding-right: 40px;
}

textarea:focus {
  box-shadow: none;
}

.send-btn {
  position: absolute;
  bottom: 0;
  right: 0;
  padding: 8px;
  background: transparent;
  color: var(--primary-color);
  border: none;
}

.send-btn:hover {
  background: var(--bg-hover);
  color: var(--primary-hover);
}

.send-btn:disabled {
  color: var(--text-secondary);
}

.model-select {
  border: none;
  background: transparent;
  font-weight: 600;
  color: var(--text-primary);
  padding-left: 0;
}
</style>
