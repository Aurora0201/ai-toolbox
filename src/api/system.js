import { invoke } from '@tauri-apps/api/core'

/**
 * System API interaction layer
 */
export const systemApi = {
  getGpuInfo: async () => {
    return await invoke('get_gpu_info')
  },

  openLogDir: async () => {
    return await invoke('open_log_dir')
  },

  setLogLevel: async (level) => {
    return await invoke('set_log_level', { level })
  }
}
