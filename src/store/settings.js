import { defineStore } from 'pinia'
import { invoke } from '@tauri-apps/api/core'
import i18n from '../i18n'

/**
 * Pinia store for managing application settings.
 * Persists data to localStorage via pinia-plugin-persistedstate.
 */
export const useSettingsStore = defineStore('settings', {
  state: () => ({
    theme: 'system', // 'light', 'dark', 'system'
    language: 'zh', // 'en', 'zh'
    ollamaEndpoint: 'http://127.0.0.1:11434',
    logLevel: 'info', // 'debug', 'info', 'warn', 'error'
    generationParameters: {
      num_ctx: 2048,
      num_predict: -1,
      temperature: 0.8,
      top_k: 40,
      top_p: 0.9,
      repeat_penalty: 1.1,
      seed: -1
    },
  }),

  actions: {
    /**
     * Initializes the application settings (theme, language, log level).
     */
    init() {
      this.applyTheme(this.theme)
      this.applyLanguage(this.language)
      this.setLogLevel(this.logLevel)
    },

    /**
     * Updates generation parameters.
     * @param {Object} params - The new parameters.
     */
    updateGenerationParameters(params) {
      this.generationParameters = { ...this.generationParameters, ...params }
    },

    /**
     * Updates the theme and applies it to the document.
     * @param {string} theme - The theme to apply ('light', 'dark', or 'system').
     */
    setTheme(theme) {
      this.theme = theme
      this.applyTheme(theme)
    },

    /**
     * Updates the log level and notifies the backend.
     * @param {string} level - The log level ('debug', 'info', 'warn', 'error').
     */
    async setLogLevel(level) {
      this.logLevel = level
      try {
        await invoke('set_log_level', { level })
      } catch (error) {
        console.error('Failed to set log level:', error)
      }
    },

    /**
     * Applies the specified theme to the HTML document.
     * @param {string} theme - The theme to apply.
     */
    applyTheme(theme) {
      const html = document.documentElement
      if (theme === 'dark' || (theme === 'system' && window.matchMedia('(prefers-color-scheme: dark)').matches)) {
        html.classList.add('dark')
      } else {
        html.classList.remove('dark')
      }
    },

    /**
     * Updates the language setting and applies it to the document.
     * @param {string} lang - The language code ('en' or 'zh').
     */
    setLanguage(lang) {
      this.language = lang
      this.applyLanguage(lang)
    },

    /**
     * Applies the language attribute to the HTML document.
     * @param {string} lang 
     */
    applyLanguage(lang) {
      document.documentElement.setAttribute('lang', lang)
      i18n.global.locale.value = lang
    },

    /**
     * Updates the Ollama endpoint and notifies the backend.
     * @param {string} endpoint - The new Ollama API endpoint.
     */
    async setOllamaEndpoint(endpoint) {
      this.ollamaEndpoint = endpoint
      try {
        await invoke('update_ollama_config', { endpoint })
      } catch (error) {
        console.error('Failed to update Ollama config in backend:', error)
      }
    },

    /**
     * Syncs current settings to the backend.
     * Useful on application startup.
     */
    async syncToBackend() {
      try {
        await invoke('update_ollama_config', { endpoint: this.ollamaEndpoint })
      } catch (error) {
        console.error('Failed to sync settings to backend:', error)
      }
    }
  },

  persist: {
    key: 'ai-toolbox-settings',
  },
})
