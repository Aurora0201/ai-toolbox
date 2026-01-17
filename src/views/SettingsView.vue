<template>
  <div class="settings-view">
    <div class="header-section mb-4">
      <h1>{{ t.title }}</h1>
      <p class="text-muted">
        {{ t.subtitle }}
      </p>
    </div>

    <!-- General Settings -->
    <div class="panel mb-4">
      <div class="panel-header">
        <span class="font-semibold">{{ t.general }}</span>
      </div>
      <div class="panel-body space-y-6">
        <!-- Theme Mode -->
        <div class="flex items-center justify-between">
          <div>
            <div class="font-medium text-primary-text">
              {{ t.themeMode }}
            </div>
            <div class="text-sm text-muted">
              {{ t.themeDesc }}
            </div>
          </div>
          <select 
            v-model="settings.theme" 
            class="w-40"
            @change="handleThemeChange"
          >
            <option value="light">
              Light
            </option>
            <option value="dark">
              Dark
            </option>
            <option value="system">
              System
            </option>
          </select>
        </div>

        <!-- Language -->
        <div class="flex items-center justify-between">
          <div>
            <div class="font-medium text-primary-text">
              {{ t.language }}
            </div>
            <div class="text-sm text-muted">
              {{ t.languageDesc }}
            </div>
          </div>
          <select 
            v-model="settings.language"
            class="w-40"
          >
            <option value="zh">
              简体中文
            </option>
            <option value="en">
              English
            </option>
          </select>
        </div>
      </div>
    </div>

    <!-- AI Connection -->
    <div class="panel mb-4">
      <div class="panel-header">
        <span class="font-semibold">{{ t.aiConnection }}</span>
      </div>
      <div class="panel-body space-y-4">
        <div>
          <label class="block text-sm font-medium mb-1 text-primary-text">{{ t.ollamaEndpoint }}</label>
          <div class="flex gap-2">
            <input 
              v-model="tempEndpoint"
              type="text" 
              placeholder="http://127.0.0.1:11434"
              class="flex-1 transition-colors"
              :class="{ 'border-red-500 focus:border-red-500 focus:ring-red-500': !isValidUrl }"
            >
            <button 
              class="btn btn-outline min-w-[160px] gap-2"
              :disabled="isChecking || !isValidUrl"
              @click="handleCheckConnection"
            >
              <span
                v-if="isChecking"
                class="loading-spinner"
              />
              <span
                v-else-if="connectionStatus === 'success'"
                class="text-success"
              >{{ t.connected }}</span>
              <span
                v-else-if="connectionStatus === 'error'"
                class="text-danger"
              >{{ t.failed }}</span>
              <span v-else>{{ t.checkConnection }}</span>
            </button>
          </div>
          <p
            v-if="!isValidUrl"
            class="text-xs text-danger mt-1"
          >
            {{ t.validUrlError }}
          </p>
          <p class="text-xs text-muted mt-1">
            {{ t.default }}: http://127.0.0.1:11434
          </p>
        </div>
        
        <div class="flex justify-end">
          <button 
            class="btn btn-primary" 
            :disabled="tempEndpoint === settings.ollamaEndpoint || !isValidUrl"
            @click="saveEndpoint"
          >
            {{ t.saveEndpoint }}
          </button>
        </div>
      </div>
    </div>

    <!-- Application Data -->
    <div class="panel mb-4">
      <div class="panel-header">
        <span class="font-semibold">{{ t.appData }}</span>
      </div>
      <div class="panel-body">
        <div class="flex items-center justify-between">
          <div>
            <div class="font-medium text-primary-text">
              {{ t.resetData }}
            </div>
            <div class="text-sm text-muted">
              {{ t.resetDesc }}
            </div>
          </div>
          <button
            class="btn btn-danger"
            @click="handleResetData"
          >
            {{ t.clearDataBtn }}
          </button>
        </div>
      </div>
    </div>

    <!-- About -->
    <div class="panel">
      <div class="panel-header">
        <span class="font-semibold">{{ t.about }}</span>
      </div>
      <div class="panel-body p-6">
        <div class="flex flex-col items-center">
          <img
            src="/tauri.svg"
            alt="App Logo"
            class="w-16 h-16 mb-4"
          >
          <h2 class="text-xl font-bold text-primary-text mb-1">
            AI Toolbox
          </h2>
          <p class="text-muted mb-6">
            {{ t.version }} {{ version }}
          </p>
          <a 
            href="https://github.com/Aurora0201/ai-toolbox" 
            target="_blank"
            class="inline-flex items-center gap-2 text-primary hover:underline transition-colors"
          >
            <svg
              class="w-5 h-5"
              fill="currentColor"
              viewBox="0 0 24 24"
            ><path d="M12 .297c-6.63 0-12 5.373-12 12 0 5.303 3.438 9.8 8.205 11.385.6.113.82-.258.82-.577 0-.285-.01-1.04-.015-2.04-3.338.724-4.042-1.61-4.042-1.61C4.422 18.07 3.633 17.7 3.633 17.7c-1.087-.744.084-.729.084-.729 1.205.084 1.838 1.236 1.838 1.236 1.07 1.835 2.809 1.305 3.495.998.108-.776.417-1.305.76-1.605-2.665-.3-5.466-1.332-5.466-5.93 0-1.31.465-2.38 1.235-3.22-.135-.303-.54-1.523.105-3.176 0 0 1.005-.322 3.3 1.23.96-.267 1.98-.399 3-.405 1.02.006 2.04.138 3 .405 2.28-1.552 3.285-1.23 3.285-1.23.645 1.653.24 2.873.12 3.176.765.84 1.23 1.91 1.23 3.22 0 4.61-2.805 5.625-5.475 5.92.43.372.823 1.102.823 2.222 0 1.606-.015 2.896-.015 3.286 0 .315.21.69.825.57C20.565 22.092 24 17.592 24 12.297c0-6.627-5.373-12-12-12" /></svg>
            {{ t.github }}
          </a>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, watch } from 'vue'
import { useSettingsStore } from '../store/settings'
import { useToast } from '../composables/useToast'
import { useConfirm } from '../composables/useConfirm'
import { invoke } from '@tauri-apps/api/core'
import packageJson from '../../package.json'

const settings = useSettingsStore()
const { addToast } = useToast()
const { confirm } = useConfirm()

const version = packageJson.version
const tempEndpoint = ref(settings.ollamaEndpoint)
const isChecking = ref(false)
const connectionStatus = ref('idle') // 'idle', 'success', 'error'

const translations = {
  en: {
    title: 'Settings',
    subtitle: 'Manage application preferences and AI connections.',
    general: 'General Settings',
    themeMode: 'Theme Mode',
    themeDesc: 'Switch between light and dark themes.',
    language: 'Language',
    languageDesc: 'Select your preferred interface language.',
    aiConnection: 'AI Connection',
    ollamaEndpoint: 'Ollama Endpoint',
    checkConnection: 'Check Connection',
    connected: '✅ Connected',
    failed: '❌ Failed',
    validUrlError: 'Please enter a valid URL.',
    default: 'Default',
    saveEndpoint: 'Save Endpoint',
    appData: 'Application Data',
    resetData: 'Reset All Data',
    resetDesc: 'Clear all token statistics and cached settings. This cannot be undone.',
    clearDataBtn: 'Clear All Data',
    about: 'About',
    version: 'Version',
    github: 'GitHub Repository',
    confirmTitle: 'Reset All Data',
    confirmMessage: 'Are you sure you want to clear all data? This will reset your token usage statistics and settings to default.',
    confirmBtn: 'Clear Everything',
    cancelBtn: 'Cancel',
    toastConnected: 'Successfully connected to Ollama',
    toastSaved: 'Ollama endpoint saved',
    toastCleared: 'All data has been cleared. App will reload.'
  },
  zh: {
    title: '设置',
    subtitle: '管理应用首选项和 AI 连接。',
    general: '通用设置',
    themeMode: '主题模式',
    themeDesc: '在浅色和深色主题之间切换。',
    language: '语言',
    languageDesc: '选择您偏好的界面语言。',
    aiConnection: 'AI 连接',
    ollamaEndpoint: 'Ollama 服务地址',
    checkConnection: '检查连接',
    connected: '✅ 已连接',
    failed: '❌ 连接失败',
    validUrlError: '请输入有效的 URL。',
    default: '默认',
    saveEndpoint: '保存地址',
    appData: '应用数据',
    resetData: '重置所有数据',
    resetDesc: '清除所有 Token 统计和缓存设置。此操作无法撤销。',
    clearDataBtn: '清除所有数据',
    about: '关于',
    version: '版本',
    github: 'GitHub 仓库',
    confirmTitle: '重置所有数据',
    confirmMessage: '您确定要清除所有数据吗？这将重置您的 Token 使用统计和设置到默认状态。',
    confirmBtn: '清除所有内容',
    cancelBtn: '取消',
    toastConnected: '成功连接到 Ollama',
    toastSaved: 'Ollama 地址已保存',
    toastCleared: '所有数据已清除。应用将重新加载。'
  }
}

const t = computed(() => translations[settings.language] || translations.en)

const isValidUrl = computed(() => {
  try {
    const url = new URL(tempEndpoint.value)
    return url.protocol === 'http:' || url.protocol === 'https:'
  } catch {
    return false
  }
})

watch(() => tempEndpoint.value, () => {
  connectionStatus.value = 'idle'
})

/**
 * Handles theme changes from the dropdown and updates the store.
 * @param {Event} event - The change event from the select element.
 */
const handleThemeChange = (event) => {
  settings.setTheme(event.target.value)
}

/**
 * Tests the connection to the Ollama endpoint using the current temporary value.
 * Temporarily updates the backend config to perform the check.
 */
const handleCheckConnection = async () => {
  if (!isValidUrl.value) return
  
  isChecking.value = true
  connectionStatus.value = 'idle'
  
  try {
    // Temporarily update backend config to check this URL
    await invoke('update_ollama_config', { endpoint: tempEndpoint.value })
    await invoke('check_connection')
    connectionStatus.value = 'success'
    addToast({ message: t.value.toastConnected, type: 'success' })
  } catch (error) {
    connectionStatus.value = 'error'
    addToast({ message: `Connection failed: ${error}`, type: 'error' })
  } finally {
    isChecking.value = false
    // Restore saved endpoint in backend if it wasn't saved to maintain state consistency
    if (tempEndpoint.value !== settings.ollamaEndpoint) {
        await invoke('update_ollama_config', { endpoint: settings.ollamaEndpoint })
    }
  }
}

/**
 * Permanently saves the current temporary Ollama endpoint to the store and backend.
 */
const saveEndpoint = async () => {
  if (!isValidUrl.value) return
  await settings.setOllamaEndpoint(tempEndpoint.value)
  addToast({ message: t.value.toastSaved, type: 'success' })
}

/**
 * Triggers a confirmation dialog to reset all application data.
 * If confirmed, clears the database and local storage.
 */
const handleResetData = async () => {
  const ok = await confirm({
    title: t.value.confirmTitle,
    message: t.value.confirmMessage,
    confirmText: t.value.confirmBtn,
    cancelText: t.value.cancelBtn
  })
  
  if (ok) {
    try {
      await invoke('clear_all_data')
      // Clear localStorage
      localStorage.removeItem('ai-toolbox-settings')
      addToast({ message: t.value.toastCleared, type: 'info' })
      setTimeout(() => {
        window.location.reload()
      }, 1500)
    } catch (error) {
      addToast({ message: `Failed to clear data: ${error}`, type: 'error' })
    }
  }
}
</script>

<style scoped>
.settings-view {
  padding: 32px;
  max-width: 900px;
  margin: 0 auto;
}

.mb-4 { margin-bottom: 24px; }
.p-6 { padding: 32px; }

.space-y-6 > * + * {
  margin-top: 24px;
}

.space-y-4 > * + * {
  margin-top: 16px;
}

.text-danger { color: var(--danger-color); }
.text-success { color: var(--success-color); }

.loading-spinner {
  width: 16px;
  height: 16px;
  border: 2px solid rgba(0,0,0,0.1);
  border-top-color: currentColor;
  border-radius: 50%;
  animation: spin 1s linear infinite;
}

@keyframes spin {
  to { transform: rotate(360deg); }
}
</style>