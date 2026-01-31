import { defineStore } from 'pinia'
import { ollamaApi } from '../api/ollama'
import { dbApi } from '../api/db'
import { useSettingsStore } from './settings' // Access settings for generation params

export const useChatStore = defineStore('chat', {
  state: () => ({
    messages: [],
    isGenerating: false,
    currentThinking: '',
    currentResponse: '',
    _stopFunction: null, // Internal reference to stop the current stream
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
      this.stopGeneration()
    },
    setGenerating(value) {
      this.isGenerating = value
    },
    
    /**
     * Sends a message to the LLM and handles the streaming response.
     * @param {Object} payload
     * @param {string} payload.text - User input text
     * @param {Array} payload.images - User input images (base64)
     * @param {string} payload.model - Selected model name
     */
    async sendMessage({ text, images, model }) {
      if (this.isGenerating) return

      const settingsStore = useSettingsStore()

      // UI Updates
      this.addMessage('user', text)
      this.setGenerating(true)
      // Add empty assistant message for streaming
      this.addMessage('assistant', '', '', false, model)

      let accumulatedContent = ''
      let accumulatedThinking = ''
      
      try {
        // Prepare options
        const options = { ...settingsStore.generationParameters }
        if (options.num_predict === -1) options.num_predict = undefined
        if (options.seed === -1) options.seed = undefined

        const requestParams = {
            request: {
                model: model,
                prompt: text,
                images: images.map(img => img.split(',')[1]),
                options: options,
                stream: true
            }
        }

        // Call API with callbacks
        this._stopFunction = await ollamaApi.generateCompletionStream(requestParams, {
            onChunk: (payload) => {
                if (payload.content) accumulatedContent += payload.content
                if (payload.thinking) accumulatedThinking += payload.thinking
                this.updateLastMessage(accumulatedContent, accumulatedThinking, payload.is_thinking)
            },
            onDone: async (payload) => {
                // Final update
                if (payload.content) accumulatedContent += payload.content
                if (payload.thinking) accumulatedThinking += payload.thinking
                this.updateLastMessage(accumulatedContent, accumulatedThinking, false)
                
                // Record Stats
                const today = new Date().toISOString().split('T')[0]
                const promptEvalCount = payload.prompt_eval_count || 0
                const evalCount = payload.eval_count || 0

                try {
                    await dbApi.recordTokens({
                        date: today,
                        prompt: promptEvalCount,
                        completion: evalCount,
                        model: model
                    })
                } catch (e) {
                    console.error('Failed to record stats:', e)
                }

                this.setGenerating(false)
                this._stopFunction = null
            }
        })

      } catch (error) {
        console.error('Generation failed:', error)
        this.updateLastMessage('Error: ' + error) // Simple error display
        this.setGenerating(false)
        this._stopFunction = null
      }
    },

    /**
     * Stops the current generation process.
     */
    async stopGeneration() {
        if (this._stopFunction) {
            this._stopFunction()
            this._stopFunction = null
        }
        if (this.isGenerating) {
             this.setGenerating(false)
             // Append stopped marker
             const lastMsg = this.messages[this.messages.length - 1]
             if (lastMsg && lastMsg.role === 'assistant') {
                 this.updateLastMessage(lastMsg.content + '\n\n*[Stopped]*', lastMsg.thinking, false)
             }
        }
    }
  }
})
