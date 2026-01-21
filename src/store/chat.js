import { defineStore } from 'pinia'

export const useChatStore = defineStore('chat', {
  state: () => ({
    messages: [],
    isGenerating: false,
    currentThinking: '',
    currentResponse: '',
  }),
  actions: {
    addMessage(role, content, thinking = '', isThinking = false) {
      this.messages.push({
        role,
        content,
        thinking,
        isThinking,
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
    }
  }
})
