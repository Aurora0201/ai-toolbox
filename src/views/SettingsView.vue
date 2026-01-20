<template>
  <div class="p-8 max-w-[900px] mx-auto">
    <div class="mb-6">
      <h1 class="text-2xl font-bold text-text-main">
        {{ t.title }}
      </h1>
      <p class="text-text-sub mt-1">
        {{ t.subtitle }}
      </p>
    </div>

    <!-- General Settings -->
    <div class="bg-background-surface border border-border rounded-lg shadow-sm mb-6">
      <div class="px-4 py-3 border-b border-border bg-background-element font-semibold text-sm text-text-main">
        <span>{{ t.general }}</span>
      </div>
      <div class="p-6 space-y-6">
        <!-- Theme Mode -->
        <div class="flex items-center justify-between">
          <div>
            <div class="font-medium text-text-main">
              {{ t.themeMode }}
            </div>
            <div class="text-sm text-text-sub">
              {{ t.themeDesc }}
            </div>
          </div>
          <select 
            v-model="settings.theme" 
            class="w-40 px-3 py-2 bg-background-surface border border-border rounded-md text-sm text-text-main focus:border-primary focus:ring-1 focus:ring-primary outline-none"
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
            <div class="font-medium text-text-main">
              {{ t.language }}
            </div>
            <div class="text-sm text-text-sub">
              {{ t.languageDesc }}
            </div>
          </div>
          <select 
            v-model="settings.language"
            class="w-40 px-3 py-2 bg-background-surface border border-border rounded-md text-sm text-text-main focus:border-primary focus:ring-1 focus:ring-primary outline-none"
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

    <!-- Log Settings -->
    <div class="bg-background-surface border border-border rounded-lg shadow-sm mb-6">
      <div class="px-4 py-3 border-b border-border bg-background-element font-semibold text-sm text-text-main">
        <span>{{ t.logSettings }}</span>
      </div>
      <div class="p-6 space-y-6">
        <!-- Log Level -->
        <div class="flex items-center justify-between">
          <div>
            <div class="font-medium text-text-main">
              {{ t.logLevel }}
            </div>
            <div class="text-sm text-text-sub">
              {{ t.logLevelDesc }}
            </div>
          </div>
          <select 
            :value="settings.logLevel" 
            class="w-40 px-3 py-2 bg-background-surface border border-border rounded-md text-sm text-text-main focus:border-primary focus:ring-1 focus:ring-primary outline-none"
            @change="handleLogLevelChange"
          >
            <option value="debug">
              Debug
            </option>
            <option value="info">
              Info
            </option>
            <option value="warn">
              Warn
            </option>
            <option value="error">
              Error
            </option>
          </select>
        </div>

        <!-- Open Log Directory -->
        <div class="flex items-center justify-between">
          <div>
            <div class="font-medium text-text-main">
              {{ t.openLogDir }}
            </div>
            <div class="text-sm text-text-sub">
              {{ t.openLogDirDesc }}
            </div>
          </div>
          <button 
            class="px-4 py-2 rounded-md font-medium text-sm transition-colors border border-border text-text-main hover:bg-background-element flex items-center gap-2"
            @click="openLogs"
          >
            <FolderOpen class="w-4 h-4" />
            {{ t.openBtn }}
          </button>
        </div>
      </div>
    </div>

    <!-- AI Connection -->
    <div class="bg-background-surface border border-border rounded-lg shadow-sm mb-6">
      <div class="px-4 py-3 border-b border-border bg-background-element font-semibold text-sm text-text-main">
        <span>{{ t.aiConnection }}</span>
      </div>
      <div class="p-6 space-y-4">
        <div>
          <label class="block text-sm font-medium mb-1 text-text-main">{{ t.ollamaEndpoint }}</label>
          <div class="flex gap-2">
            <input 
              v-model="tempEndpoint"
              type="text" 
              placeholder="http://127.0.0.1:11434"
              class="flex-1 px-3 py-2 border border-border rounded-md bg-background-surface text-sm text-text-main outline-none focus:border-primary focus:ring-1 focus:ring-primary transition-colors"
              :class="{ 'border-danger focus:border-danger focus:ring-danger': !isValidUrl }"
            >
            <button 
              class="px-4 py-2 rounded-md font-medium text-sm transition-colors disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center gap-2 min-w-[160px] border border-border text-text-main hover:bg-background-element"
              :disabled="isChecking || !isValidUrl"
              @click="handleCheckConnection"
            >
              <Loader2
                v-if="isChecking"
                class="w-4 h-4 animate-spin"
              />
              <span
                v-else-if="connectionStatus === 'success'"
                class="text-success font-semibold"
              >{{ t.connected }}</span>
              <span
                v-else-if="connectionStatus === 'error'"
                class="text-danger font-semibold"
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
          <p class="text-xs text-text-sub mt-1">
            {{ t.default }}: http://127.0.0.1:11434
          </p>
        </div>
        
        <div class="flex justify-end">
          <button 
            class="px-4 py-2 rounded-md font-medium text-sm transition-colors disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center gap-2 bg-primary text-white hover:bg-primary-hover" 
            :disabled="tempEndpoint === settings.ollamaEndpoint || !isValidUrl"
            @click="saveEndpoint"
          >
            {{ t.saveEndpoint }}
          </button>
        </div>
      </div>
    </div>

    <!-- Application Data -->
    <div class="bg-background-surface border border-border rounded-lg shadow-sm mb-6">
      <div class="px-4 py-3 border-b border-border bg-background-element font-semibold text-sm text-text-main">
        <span>{{ t.appData }}</span>
      </div>
      <div class="p-6">
        <div class="flex items-center justify-between">
          <div>
            <div class="font-medium text-text-main">
              {{ t.resetData }}
            </div>
            <div class="text-sm text-text-sub">
              {{ t.resetDesc }}
            </div>
          </div>
          <button
            class="px-4 py-2 rounded-md font-medium text-sm transition-colors disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center gap-2 bg-danger text-white hover:opacity-90"
            @click="handleResetData"
          >
            {{ t.clearDataBtn }}
          </button>
        </div>
      </div>
    </div>

    <!-- About -->
    <div class="bg-background-surface border border-border rounded-lg shadow-sm">
      <div class="px-4 py-3 border-b border-border bg-background-element font-semibold text-sm text-text-main">
        <span>{{ t.about }}</span>
      </div>
      <div class="p-6">
        <div class="flex flex-col items-center">
          <img
            src="/tauri.svg"
            alt="App Logo"
            class="w-16 h-16 mb-4"
          >
          <h2 class="text-xl font-bold text-text-main mb-1">
            AI Toolbox
          </h2>
          <p class="text-text-sub mb-6">
            {{ t.version }} {{ version }}
          </p>
          <a 
            href="https://github.com/Aurora0201/ai-toolbox" 
            target="_blank"
            class="inline-flex items-center gap-2 text-primary hover:underline transition-colors font-medium text-sm"
          >
            <Github class="w-5 h-5" />
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
import { Loader2, Github, FolderOpen } from 'lucide-vue-next'

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
    logSettings: 'Log Settings',
    logLevel: 'Log Level',
    logLevelDesc: 'Set the verbosity of application logs (Retained for 7 days).',
    openLogDir: 'Open Log Directory',
    openLogDirDesc: 'Open the folder containing application logs.',
    openBtn: 'Open Folder',
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
    logSettings: '日志设置',
    logLevel: '日志等级',
    logLevelDesc: '设置应用程序日志的详细程度（保留 7 天）。',
    openLogDir: '打开日志目录',
    openLogDirDesc: '打开包含应用程序日志的文件夹。',
    openBtn: '打开文件夹',
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
 * Handles log level changes from the dropdown and updates the store.
 * @param {Event} event - The change event from the select element.
 */
const handleLogLevelChange = (event) => {
  settings.setLogLevel(event.target.value)
}

/**
 * Opens the application log directory.
 */
const openLogs = async () => {
  try {
    await invoke('open_log_dir')
  } catch (error) {
    addToast({ message: `Failed to open log directory: ${error}`, type: 'error' })
  }
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