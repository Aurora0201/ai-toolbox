<template>
  <div class="flex flex-col gap-6">
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
import { ref, onMounted, onUnmounted } from 'vue'
import { useModelStore } from '../store/models'
import { useToast } from '../composables/useToast'
import { useConfirm } from '../composables/useConfirm'
import ModelList from './models/ModelList.vue'
import RunningProcesses from './models/RunningProcesses.vue'
import { useI18n } from 'vue-i18n'

/**
 * ModelManager is the main container for model-related UI components.
 * It orchestrates actions between the store and sub-components.
 */
const store = useModelStore()
const { addToast } = useToast()
const { confirm } = useConfirm()
const { t } = useI18n()

const pulling = ref(false)
const loadingStates = ref({}) // map of modelName -> 'starting' | 'stopping' | 'deleting'

/**
 * Pulls a new model from Ollama.
 * @param {string} name 
 */
const pullModel = async (name) => {
  pulling.value = true
  addToast({ message: t('models.pullingMsg', { name }), type: 'info' })
  try {
    await store.pullModel(name)
    addToast({ message: t('models.pullSuccess', { name }), type: 'success' })
  } catch (error) {
    addToast({ message: t('models.pullFailed') + error, type: 'error' })
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
  loadingStates.value = { ...loadingStates.value, [name]: 'starting' }
  try {
    await store.startModel(name)
    addToast({ message: t('models.started', { name }), type: 'success' })
  } catch (error) {
    addToast({ message: t('models.startFailed') + error, type: 'error' })
  } finally {
    const nextStates = { ...loadingStates.value }
    delete nextStates[name]
    loadingStates.value = nextStates
  }
}

/**
 * Unloads a model from memory to free VRAM.
 * @param {string} name 
 */
const stopModel = async (name) => {
  if (loadingStates.value[name]) return
  loadingStates.value = { ...loadingStates.value, [name]: 'stopping' }
  try {
    await store.unloadModel(name)
    addToast({ message: t('models.stopped', { name }), type: 'success' })
  } catch (error) {
    addToast({ message: t('models.stopFailed') + error, type: 'error' })
  } finally {
    const nextStates = { ...loadingStates.value }
    delete nextStates[name]
    loadingStates.value = nextStates
  }
}

/**
 * Deletes a model from the local library.
 * @param {string} name 
 */
const deleteModel = async (name) => {
  const confirmed = await confirm({
    title: t('models.confirmDeleteTitle'),
    message: t('models.confirmDeleteMsg', { name }),
    confirmText: t('common.delete'),
    cancelText: t('common.cancel')
  })
  
  if (!confirmed) return
  
  if (loadingStates.value[name]) return
  loadingStates.value = { ...loadingStates.value, [name]: 'deleting' }
  try {
    await store.deleteModel(name)
    addToast({ message: t('models.deleted', { name }), type: 'success' })
  } catch (error) {
    addToast({ message: t('models.deleteFailed') + error, type: 'error' })
  } finally {
    const nextStates = { ...loadingStates.value }
    delete nextStates[name]
    loadingStates.value = nextStates
  }
}

// Initial data fetch and periodic updates for running status
onMounted(() => {
  store.fetchModels()
  store.setupPullListener()
  store.startMonitoring()
})

onUnmounted(() => {
  store.stopMonitoring()
})
</script>