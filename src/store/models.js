import { defineStore } from 'pinia'
import { listen } from '@tauri-apps/api/event'
import i18n from '../i18n'
import { ollamaApi } from '../api/ollama'
import { systemApi } from '../api/system'

/**
 * Pinia store for managing Ollama models, running processes, and system resource info.
 */
export const useModelStore = defineStore('models', {
  state: () => ({
    models: [],           // List of installed models
    runningModels: [],    // List of models currently in VRAM
    loading: false,       // Loading state for model list
    selectedModel: '',    // Currently selected model for chat
    gpuInfo: { name: '', total: 0, used: 0 }, // GPU resource usage
    pullProgress: { status: '', completed: 0, total: 0, percentage: 0 }, // Progress of current model pull
    _listenersReady: false, // Internal flag to prevent multiple event listeners
  }),

  actions: {
    /**
     * Fetches the list of all installed models from the Ollama server.
     */
    async fetchModels() {
      this.loading = true
      try {
        this.models = await ollamaApi.getModels()
      } catch (error) {
        console.error('Failed to fetch models:', error)
      } finally {
        this.loading = false
      }
    },
    
    /**
     * Fetches GPU information and VRAM usage from the system.
     */
    async fetchGpuInfo() {
      try {
        const info = await systemApi.getGpuInfo()
        // Backend returns MB, convert to Bytes for consistent formatting in frontend
        this.gpuInfo = {
          name: info.name,
          total: info.total_mb * 1024 * 1024,
          used: info.used_mb * 1024 * 1024
        }
      } catch (error) {
        console.warn('Failed to fetch GPU info:', error)
      }
    },

    /**
     * Fetches the list of models currently loaded in memory.
     */
    async fetchRunningModels() {
      try {
        this.runningModels = await ollamaApi.getRunningModels()
      } catch (error) {
        console.error('Failed to fetch running models:', error)
      }
    },

    /**
     * Sets up a listener for model pull progress events.
     */
    async setupPullListener() {
      await listen('pull-progress', (event) => {
        const { status, completed, total } = event.payload
        let percentage = 0
        if (total > 0) {
          percentage = Math.round((completed / total) * 100)
        }
        this.pullProgress = { status, completed, total, percentage }
      })
    },

    /**
     * Sets up listeners for backend monitoring events.
     */
    async setupStatusListeners() {
      // Listen for running models updates
      await listen('running-models-update', (event) => {
        this.runningModels = event.payload
      })

      // Listen for GPU info updates
      await listen('gpu-info-update', (event) => {
        const info = event.payload
        this.gpuInfo = {
          name: info.name,
          total: info.total_mb * 1024 * 1024,
          used: info.used_mb * 1024 * 1024
        }
      })
    },

    /**
     * Pulls (downloads) a new model by name.
     * @param {string} name - The name of the model to pull (e.g., "llama3").
     */
    async pullModel(name) {
      this.pullProgress = { status: i18n.global.t('common.loading'), completed: 0, total: 0, percentage: 0 }
      try {
        await ollamaApi.pullModel(name)
        await this.fetchModels()
      } catch (error) {
        console.error('Failed to pull model:', error)
        throw error
      } finally {
        this.pullProgress = { status: '', completed: 0, total: 0, percentage: 0 }
      }
    },

    /**
     * Deletes an installed model.
     * @param {string} name - The name of the model to delete.
     */
    async deleteModel(name) {
      try {
        await ollamaApi.deleteModel(name)
        await this.fetchModels()
      } catch (error) {
        console.error('Failed to delete model:', error)
        throw error
      }
    },

    /**
     * Starts (preloads) a model into VRAM.
     * @param {string} name - The name of the model to start.
     */
    async startModel(name) {
      try {
        await ollamaApi.startModel(name)
        await this.fetchRunningModels()
      } catch (error) {
        console.error('Failed to start model:', error)
        throw error
      }
    },

    /**
     * Unloads a model from memory to free up VRAM.
     * @param {string} name - The name of the model to unload.
     */
    async unloadModel(name) {
      try {
        await ollamaApi.unloadModel(name)
        await this.fetchRunningModels()
      } catch (error) {
        console.error('Failed to unload model:', error)
        throw error
      }
    },

    /**
     * Selects a model to be used for chat sessions.
     * @param {string} name - The name of the model to select.
     */
    selectModel(name) {
      this.selectedModel = name
      localStorage.setItem('selectedModel', name)
    },

    /**
     * Starts monitoring for model and system status.
     * Uses backend-driven events instead of frontend polling.
     */
    async startMonitoring() {
      if (this._listenersReady) return
      
      // Initial fetch to get current state immediately
      this.fetchRunningModels()
      this.fetchGpuInfo()
      
      await this.setupStatusListeners()
      this._listenersReady = true
    },

    /**
     * Stops the periodic polling.
     * In the event-driven model, this is mostly a no-op as the backend
     * continues to monitor, but we keep the method for API compatibility.
     */
    stopMonitoring() {
      // Listeners remain active for the life of the store
    }
  }
})
