import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'

/**
 * Ollama API interaction layer
 */
export const ollamaApi = {
  getModels: async () => {
    return await invoke('get_models')
  },

  getRunningModels: async () => {
    return await invoke('get_running_models')
  },

  pullModel: async (name) => {
    return await invoke('pull_model', { name })
  },

  deleteModel: async (name) => {
    return await invoke('delete_model', { name })
  },

  startModel: async (name) => {
    return await invoke('start_model', { name })
  },

  unloadModel: async (name) => {
    return await invoke('unload_model', { name })
  },

  generateCompletion: async (params) => {
    return await invoke('generate_completion', params)
  },

  /**
   * Generates completion with streaming support.
   * Encapsulates the event listening and invocation.
   * @param {Object} params - The request parameters for generate_completion.
   * @param {Object} callbacks - Callbacks for stream events.
   * @param {Function} callbacks.onChunk - Called when a chunk is received.
   * @param {Function} callbacks.onDone - Called when generation is done.
   * @returns {Promise<Function>} - A function to stop/unlisten.
   */
  generateCompletionStream: async (params, { onChunk, onDone }) => {
    let unlisten = null;
    
    // Setup listener
    unlisten = await listen('chat-response', (event) => {
      const payload = event.payload
      if (payload.done) {
        if (onDone) onDone(payload)
        if (unlisten) {
            unlisten()
            unlisten = null
        }
      } else {
        if (onChunk) onChunk(payload)
      }
    })

    try {
      await invoke('generate_completion', params)
    } catch (error) {
      if (unlisten) {
          unlisten()
          unlisten = null
      }
      throw error
    }

    // Return a stop function
    return () => {
      if (unlisten) {
        unlisten()
        unlisten = null
      }
    }
  },

  updateConfig: async (endpoint) => {
    return await invoke('update_ollama_config', { endpoint })
  },

  checkConnection: async () => {
    return await invoke('check_connection')
  }
}
