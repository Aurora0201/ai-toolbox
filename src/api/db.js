import { invoke } from '@tauri-apps/api/core'

/**
 * Database API interaction layer
 */
export const dbApi = {
  recordTokens: async (params) => {
    return await invoke('record_tokens', params)
  },

  getTokenStats: async () => {
    return await invoke('get_token_stats')
  },

  clearAllData: async () => {
    return await invoke('clear_all_data')
  }
}
