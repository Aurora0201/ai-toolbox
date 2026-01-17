<template>
  <div class="model-manager">
    <!-- Component for managing installed models -->
    <ModelList 
      :models="store.models" 
      :running-models="store.runningModels"
      :loading="store.loading"
      :pulling="pulling"
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
import { ref, onMounted } from 'vue'
import { useModelStore } from '../store/models'
import { useToast } from '../composables/useToast'
import ModelList from './models/ModelList.vue'
import RunningProcesses from './models/RunningProcesses.vue'

/**
 * ModelManager is the main container for model-related UI components.
 * It orchestrates actions between the store and sub-components.
 */
const store = useModelStore()
const { addToast } = useToast()

const pulling = ref(false)
const loadingStates = ref({}) // map of modelName -> 'starting' | 'stopping' | 'deleting'

/**
 * Pulls a new model from Ollama.
 * @param {string} name 
 */
const pullModel = async (name) => {
  pulling.value = true
  addToast({ message: `Pulling model ${name}...`, type: 'info' })
  try {
    await store.pullModel(name)
    addToast({ message: `Successfully pulled ${name}`, type: 'success' })
  } catch (error) {
    addToast({ message: 'Failed to pull model: ' + error, type: 'error' })
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
    addToast({ message: `Started ${name}`, type: 'success' })
  } catch (error) {
    addToast({ message: 'Failed to start: ' + error, type: 'error' })
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
    addToast({ message: `Stopped ${name}`, type: 'success' })
  } catch (error) {
    addToast({ message: 'Failed to stop: ' + error, type: 'error' })
  } finally {
    loadingStates.value[name] = null
  }
}

/**
 * Deletes a model from the local library.
 * @param {string} name 
 */
const deleteModel = async (name) => {
  if (!confirm(`Are you sure you want to delete ${name}?`)) return
  if (loadingStates.value[name]) return
  loadingStates.value[name] = 'deleting'
  try {
    await store.deleteModel(name)
    addToast({ message: `Deleted ${name}`, type: 'success' })
  } catch (error) {
    addToast({ message: 'Failed to delete: ' + error, type: 'error' })
  } finally {
    loadingStates.value[name] = null
  }
}

// Initial data fetch and periodic updates for running status
onMounted(() => {
  store.fetchModels()
  store.fetchRunningModels()
  store.fetchGpuInfo()
  
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