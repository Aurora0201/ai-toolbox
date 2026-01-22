import { defineStore } from 'pinia'

export const useTranslatorStore = defineStore('translator', {
  state: () => ({
    messages: [],
    isGenerating: false,
    targetLanguage: 'Chinese', // Default target
  }),
  actions: {
    addMessage(role, content, thinking = '', isThinking = false, model = '') {
      this.messages.push({
        role,
        content,
        thinking,
        isThinking,
        model,
        timestamp: Date.now()
      })
    },
    updateLastMessage(content, thinking = '', isThinking = false) {
      if (this.messages.length > 0) {
        const lastMsg = this.messages[this.messages.length - 1]
        if (lastMsg.role === 'assistant') {
          lastMsg.content = content
          lastMsg.thinking = thinking
          lastMsg.isThinking = isThinking
        }
      }
    },
    clearMessages() {
      this.messages = []
    },
    setGenerating(value) {
      this.isGenerating = value
    },
    setTargetLanguage(lang) {
      this.targetLanguage = lang
    }
  },
  persist: {
    key: 'ai-toolbox-translator',
    paths: ['targetLanguage']
  }
})
