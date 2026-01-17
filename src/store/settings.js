import { defineStore } from 'pinia'
import { invoke } from '@tauri-apps/api/core'

/**
 * Pinia store for managing application settings.
 * Persists data to localStorage via pinia-plugin-persistedstate.
 */
export const useSettingsStore = defineStore('settings', {
  state: () => ({
    theme: 'system', // 'light', 'dark', 'system'
    language: 'zh', // 'en', 'zh'
    ollamaEndpoint: 'http://127.0.0.1:11434',
  }),

  actions: {
    /**
     * Initializes the application settings (theme and language).
     */
    init() {
      this.applyTheme(this.theme)
      this.applyLanguage(this.language)
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
