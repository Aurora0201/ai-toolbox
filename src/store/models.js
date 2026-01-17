import { defineStore } from 'pinia'
import { invoke } from '@tauri-apps/api/core'

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
  }),

  actions: {
    /**
     * Fetches the list of all installed models from the Ollama server.
     */
    async fetchModels() {
      this.loading = true
      try {
        this.models = await invoke('get_models')
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
        const info = await invoke('get_gpu_info')
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
        this.runningModels = await invoke('get_running_models')
      } catch (error) {
        console.error('Failed to fetch running models:', error)
      }
    },

    /**
     * Pulls (downloads) a new model by name.
     * @param {string} name - The name of the model to pull (e.g., "llama3").
     */
    async pullModel(name) {
      try {
        await invoke('pull_model', { name })
        await this.fetchModels()
      } catch (error) {
        console.error('Failed to pull model:', error)
        throw error
      }
    },

    /**
     * Deletes an installed model.
     * @param {string} name - The name of the model to delete.
     */
    async deleteModel(name) {
      try {
        await invoke('delete_model', { name })
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
        await invoke('start_model', { name })
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
        await invoke('unload_model', { name })
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
    }
  }
})
