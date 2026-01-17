<template>
  <div class="model-manager">
    <!-- Component for managing installed models -->
    <ModelList 
      :models="store.models" 
      :running-models="store.runningModels"
      :loading="store.loading"
      :pulling="pulling"
      :pull-progress="store.pullProgress"
      :loading-states="loadingStates"
      @pull="pullModel"
      @start="startModel"
      @delete="deleteModel"
    />

    <!-- Component for monitoring running processes and resources -->
    <RunningProcesses 
      :running-models="store.runningModels"
      :gpu-info="store.gpuInfo"
      :loading-states="loadingStates"
      @stop="stopModel"
    />
  </div>
</template>

<script setup>
import { ref, onMounted, computed } from 'vue'
import { useModelStore } from '../store/models'
import { useSettingsStore } from '../store/settings'
import { useToast } from '../composables/useToast'
import { useConfirm } from '../composables/useConfirm'
import ModelList from './models/ModelList.vue'
import RunningProcesses from './models/RunningProcesses.vue'

/**
 * ModelManager is the main container for model-related UI components.
 * It orchestrates actions between the store and sub-components.
 */
const store = useModelStore()
const settings = useSettingsStore()
const { addToast } = useToast()
const { confirm } = useConfirm()

const pulling = ref(false)
const loadingStates = ref({}) // map of modelName -> 'starting' | 'stopping' | 'deleting'

const translations = {
  en: {
    pulling: 'Pulling model {name}...',
    pullSuccess: 'Successfully pulled {name}',
    pullFailed: 'Failed to pull model: ',
    started: 'Started {name}',
    startFailed: 'Failed to start: ',
    stopped: 'Stopped {name}',
    stopFailed: 'Failed to stop: ',
    deleted: 'Deleted {name}',
    deleteFailed: 'Failed to delete: ',
    confirmDeleteTitle: 'Delete Model',
    confirmDeleteMsg: 'Are you sure you want to delete {name}? This action cannot be undone.',
    deleteBtn: 'Delete',
    cancelBtn: 'Cancel'
  },
  zh: {
    pulling: '正在拉取模型 {name}...',
    pullSuccess: '成功拉取 {name}',
    pullFailed: '拉取模型失败: ',
    started: '已启动 {name}',
    startFailed: '启动失败: ',
    stopped: '已停止 {name}',
    stopFailed: '停止失败: ',
    deleted: '已删除 {name}',
    deleteFailed: '删除失败: ',
    confirmDeleteTitle: '删除模型',
    confirmDeleteMsg: '您确定要删除 {name} 吗？此操作无法撤销。',
    deleteBtn: '删除',
    cancelBtn: '取消'
  }
}

const t = computed(() => translations[settings.language] || translations.en)

/**
 * Pulls a new model from Ollama.
 * @param {string} name 
 */
const pullModel = async (name) => {
  pulling.value = true
  addToast({ message: t.value.pulling.replace('{name}', name), type: 'info' })
  try {
    await store.pullModel(name)
    addToast({ message: t.value.pullSuccess.replace('{name}', name), type: 'success' })
  } catch (error) {
    addToast({ message: t.value.pullFailed + error, type: 'error' })
  } finally {
    pulling.value = false
  }
}

/**
 * Loads a model into memory.
 * @param {string} name 
 */
const startModel = async (name) => {
  if (loadingStates.value[name]) return
  loadingStates.value[name] = 'starting'
  try {
    await store.startModel(name)
    addToast({ message: t.value.started.replace('{name}', name), type: 'success' })
  } catch (error) {
    addToast({ message: t.value.startFailed + error, type: 'error' })
  } finally {
    loadingStates.value[name] = null
  }
}

/**
 * Unloads a model from memory to free VRAM.
 * @param {string} name 
 */
const stopModel = async (name) => {
  if (loadingStates.value[name]) return
  loadingStates.value[name] = 'stopping'
  try {
    await store.unloadModel(name)
    addToast({ message: t.value.stopped.replace('{name}', name), type: 'success' })
  } catch (error) {
    addToast({ message: t.value.stopFailed + error, type: 'error' })
  } finally {
    loadingStates.value[name] = null
  }
}

/**
 * Deletes a model from the local library.
 * @param {string} name 
 */
const deleteModel = async (name) => {
  const confirmed = await confirm({
    title: t.value.confirmDeleteTitle,
    message: t.value.confirmDeleteMsg.replace('{name}', name),
    confirmText: t.value.deleteBtn,
    cancelText: t.value.cancelBtn
  })
  
  if (!confirmed) return
  
  if (loadingStates.value[name]) return
  loadingStates.value[name] = 'deleting'
  try {
    await store.deleteModel(name)
    addToast({ message: t.value.deleted.replace('{name}', name), type: 'success' })
  } catch (error) {
    addToast({ message: t.value.deleteFailed + error, type: 'error' })
  } finally {
    loadingStates.value[name] = null
  }
}

// Initial data fetch and periodic updates for running status
onMounted(() => {
  store.fetchModels()
  store.fetchRunningModels()
  store.fetchGpuInfo()
  store.setupPullListener()
  
  // Refresh running models and GPU info every 5 seconds
  const interval = setInterval(() => {
    store.fetchRunningModels()
    store.fetchGpuInfo()
  }, 5000)
  
  // Cleanup on unmount (not strictly necessary for setInterval in this context but good practice)
  return () => clearInterval(interval)
})
</script>

<style scoped>
.model-manager {
  display: flex;
  flex-direction: column;
  gap: 24px;
}
</style>