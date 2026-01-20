<template>
  <div class="p-8 max-w-[900px] mx-auto">
    <div class="mb-6">
      <h1 class="text-2xl font-bold text-text-main">
        {{ $t('settings.title') }}
      </h1>
      <p class="text-text-sub mt-1">
        {{ $t('settings.subtitle') }}
      </p>
    </div>

    <!-- General Settings -->
    <div class="bg-background-surface border border-border rounded-lg shadow-sm mb-6 overflow-hidden">
      <div class="px-4 py-3 border-b border-border bg-background-element font-semibold text-sm text-text-main">
        <span>{{ $t('settings.general') }}</span>
      </div>
      <div class="p-6 space-y-6">
        <!-- Theme Mode -->
        <div class="flex items-center justify-between">
          <div>
            <div class="font-medium text-text-main">
              {{ $t('settings.themeMode') }}
            </div>
            <div class="text-sm text-text-sub">
              {{ $t('settings.themeDesc') }}
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
              {{ $t('settings.language') }}
            </div>
            <div class="text-sm text-text-sub">
              {{ $t('settings.languageDesc') }}
            </div>
          </div>
          <select 
            v-model="settings.language"
            class="w-40 px-3 py-2 bg-background-surface border border-border rounded-md text-sm text-text-main focus:border-primary focus:ring-1 focus:ring-primary outline-none"
            @change="handleLanguageChange"
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
    <div class="bg-background-surface border border-border rounded-lg shadow-sm mb-6 overflow-hidden">
      <div class="px-4 py-3 border-b border-border bg-background-element font-semibold text-sm text-text-main">
        <span>{{ $t('settings.logSettings') }}</span>
      </div>
      <div class="p-6 space-y-6">
        <!-- Log Level -->
        <div class="flex items-center justify-between">
          <div>
            <div class="font-medium text-text-main">
              {{ $t('settings.logLevel') }}
            </div>
            <div class="text-sm text-text-sub">
              {{ $t('settings.logLevelDesc') }}
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
              {{ $t('settings.openLogDir') }}
            </div>
            <div class="text-sm text-text-sub">
              {{ $t('settings.openLogDirDesc') }}
            </div>
          </div>
          <button 
            class="px-4 py-2 rounded-md font-medium text-sm transition-colors border border-border text-text-main hover:bg-background-element flex items-center gap-2"
            @click="openLogs"
          >
            <FolderOpen class="w-4 h-4" />
            {{ $t('settings.openBtn') }}
          </button>
        </div>
      </div>
    </div>

    <!-- AI Connection -->
    <div class="bg-background-surface border border-border rounded-lg shadow-sm mb-6 overflow-hidden">
      <div class="px-4 py-3 border-b border-border bg-background-element font-semibold text-sm text-text-main">
        <span>{{ $t('settings.aiConnection') }}</span>
      </div>
      <div class="p-6 space-y-4">
        <div>
          <label class="block text-sm font-medium mb-1 text-text-main">{{ $t('settings.ollamaEndpoint') }}</label>
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
              >{{ $t('settings.connected') }}</span>
              <span
                v-else-if="connectionStatus === 'error'"
                class="text-danger font-semibold"
              >{{ $t('settings.failed') }}</span>
              <span v-else>{{ $t('settings.checkConnection') }}</span>
            </button>
          </div>
          <p
            v-if="!isValidUrl"
            class="text-xs text-danger mt-1"
          >
            {{ $t('settings.validUrlError') }}
          </p>
          <p class="text-xs text-text-sub mt-1">
            {{ $t('settings.default') }}: http://127.0.0.1:11434
          </p>
        </div>
        
        <div class="flex justify-end">
          <button 
            class="px-4 py-2 rounded-md font-medium text-sm transition-colors disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center gap-2 bg-primary text-white hover:bg-primary-hover" 
            :disabled="tempEndpoint === settings.ollamaEndpoint || !isValidUrl"
            @click="saveEndpoint"
          >
            {{ $t('settings.saveEndpoint') }}
          </button>
        </div>
      </div>
    </div>

    <!-- Application Data -->
    <div class="bg-background-surface border border-border rounded-lg shadow-sm mb-6 overflow-hidden">
      <div class="px-4 py-3 border-b border-border bg-background-element font-semibold text-sm text-text-main">
        <span>{{ $t('settings.appData') }}</span>
      </div>
      <div class="p-6">
        <div class="flex items-center justify-between">
          <div>
            <div class="font-medium text-text-main">
              {{ $t('settings.resetData') }}
            </div>
            <div class="text-sm text-text-sub">
              {{ $t('settings.resetDesc') }}
            </div>
          </div>
          <button
            class="px-4 py-2 rounded-md font-medium text-sm transition-colors disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center gap-2 bg-danger text-white hover:opacity-90"
            @click="handleResetData"
          >
            {{ $t('settings.clearDataBtn') }}
          </button>
        </div>
      </div>
    </div>

    <!-- About -->
    <div class="bg-background-surface border border-border rounded-lg shadow-sm overflow-hidden">
      <div class="px-4 py-3 border-b border-border bg-background-element font-semibold text-sm text-text-main">
        <span>{{ $t('settings.about') }}</span>
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
            {{ $t('settings.version') }} {{ version }}
          </p>
          <a 
            href="https://github.com/Aurora0201/ai-toolbox" 
            target="_blank"
            class="inline-flex items-center gap-2 text-primary hover:underline transition-colors font-medium text-sm"
          >
            <Github class="w-5 h-5" />
            {{ $t('settings.github') }}
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

import { useI18n } from 'vue-i18n'



const settings = useSettingsStore()

const { addToast } = useToast()

const { confirm } = useConfirm()

const { t } = useI18n()



const version = packageJson.version

const tempEndpoint = ref(settings.ollamaEndpoint)

const isChecking = ref(false)

const connectionStatus = ref('idle') // 'idle', 'success', 'error'



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

 * Handles language changes from the dropdown and updates the store.

 * @param {Event} event - The change event from the select element.

 */

const handleLanguageChange = (event) => {

  settings.setLanguage(event.target.value)

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

    addToast({ message: t('settings.toastConnected'), type: 'success' })

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

  addToast({ message: t('settings.toastSaved'), type: 'success' })

}



/**

 * Triggers a confirmation dialog to reset all application data.

 * If confirmed, clears the database and local storage.

 */

const handleResetData = async () => {

  const ok = await confirm({

    title: t('settings.confirmTitle'),

    message: t('settings.confirmMessage'),

    confirmText: t('settings.confirmBtn'),

    cancelText: t('common.cancel')

  })

  

  if (ok) {

    try {

      await invoke('clear_all_data')

      // Clear localStorage

      localStorage.removeItem('ai-toolbox-settings')

      addToast({ message: t('settings.toastCleared'), type: 'info' })

      setTimeout(() => {

        window.location.reload()

      }, 1500)

    } catch (error) {

      addToast({ message: `Failed to clear data: ${error}`, type: 'error' })

    }

  }

}

</script>
