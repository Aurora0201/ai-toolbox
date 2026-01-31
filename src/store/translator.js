import { defineStore } from 'pinia'
import { ollamaApi } from '../api/ollama'
import { dbApi } from '../api/db'
import { useSettingsStore } from './settings'

export const useTranslatorStore = defineStore('translator', {
  state: () => ({
    messages: [],
    isGenerating: false,
    targetLanguage: 'Chinese', // Default target
    _stopFunction: null
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
      this.stopTranslation()
    },
    setGenerating(value) {
      this.isGenerating = value
    },
    setTargetLanguage(lang) {
      this.targetLanguage = lang
    },

    /**
     * Translates text and handles the streaming response.
     */
    async translate({ text, model }) {
       if (this.isGenerating) return

       const settingsStore = useSettingsStore()

       this.addMessage('user', text)
       this.setGenerating(true)
       this.addMessage('assistant', '', '', false, model)

       let accumulatedContent = ''
       let accumulatedThinking = ''

       try {
           const options = { ...settingsStore.generationParameters }
           if (options.num_predict === -1) options.num_predict = undefined
           if (options.seed === -1) options.seed = undefined

           const prompt = `Translate the following text into ${this.targetLanguage}. Do not provide any explanations, notes, or introductions. Just provide the translated text.\n\nText to translate:\n${text}`

           const requestParams = {
               request: {
                   model: model,
                   prompt: prompt,
                   images: [],
                   options: options,
                   stream: true
               }
           }

           this._stopFunction = await ollamaApi.generateCompletionStream(requestParams, {
               onChunk: (payload) => {
                   if (payload.content) accumulatedContent += payload.content
                   if (payload.thinking) accumulatedThinking += payload.thinking
                   this.updateLastMessage(accumulatedContent, accumulatedThinking, payload.is_thinking)
               },
               onDone: async (payload) => {
                   if (payload.content) accumulatedContent += payload.content
                   if (payload.thinking) accumulatedThinking += payload.thinking
                   this.updateLastMessage(accumulatedContent, accumulatedThinking, false)
                   
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
           console.error('Translation failed:', error)
           this.updateLastMessage('Error: ' + error)
           this.setGenerating(false)
           this._stopFunction = null
       }
    },

    async stopTranslation() {
        if (this._stopFunction) {
            this._stopFunction()
            this._stopFunction = null
        }
        if (this.isGenerating) {
             this.setGenerating(false)
             const lastMsg = this.messages[this.messages.length - 1]
             if (lastMsg && lastMsg.role === 'assistant') {
                 this.updateLastMessage(lastMsg.content + '\n\n*[Stopped]*', lastMsg.thinking, false)
             }
        }
    }
  },
  persist: {
    key: 'ai-toolbox-translator',
    paths: ['targetLanguage']
  }
})
