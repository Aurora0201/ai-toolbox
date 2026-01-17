<template>
  <div class="panel mb-4">
    <div class="panel-header">
      <span class="d-flex gap-2">📦 Installed Models</span>
      <span
        class="text-muted text-mono"
        style="font-size: 12px"
      >{{ models.length }} items</span>
    </div>
    <div class="panel-body">
      <!-- Pull Model Input -->
      <div class="d-flex gap-2 mb-3">
        <input 
          v-model="newModelName" 
          placeholder="Pull model (e.g. llama3)" 
          style="flex: 1"
          @keyup.enter="handlePull"
        >
        <button
          :disabled="pulling"
          class="btn btn-primary d-flex gap-2"
          @click="handlePull"
        >
          <svg
            xmlns="http://www.w3.org/2000/svg"
            width="16"
            height="16"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
          ><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" /><polyline points="7 10 12 15 17 10" /><line
            x1="12"
            y1="15"
            x2="12"
            y2="3"
          /></svg>
          {{ pulling ? 'Pulling...' : 'Pull' }}
        </button>
      </div>
      
      <!-- Models List -->
      <div
        v-if="loading"
        class="text-center p-3 text-muted"
      >
        <div class="spinner mb-2">
          ⏳
        </div>
        Loading library...
      </div>
      <div
        v-else
        class="list-group"
      >
        <ModelItem 
          v-for="model in models" 
          :key="model.name" 
          :model="model"
          :is-running="isModelRunning(model.name)"
          :loading-state="loadingStates[model.name]"
          @start="$emit('start', $event)"
          @delete="$emit('delete', $event)"
        />
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue'
import ModelItem from './ModelItem.vue'

/**
 * ModelList component manages the display of installed models and the pull interface.
 */
const props = defineProps({
  models: {
    type: Array,
    required: true
  },
  runningModels: {
    type: Array,
    required: true
  },
  loading: {
    type: Boolean,
    default: false
  },
  pulling: {
    type: Boolean,
    default: false
  },
  loadingStates: {
    type: Object,
    default: () => ({})
  }
})

const emit = defineEmits(['pull', 'start', 'delete'])

const newModelName = ref('')

/**
 * Handles the pull model action.
 */
const handlePull = () => {
  if (!newModelName.value) return
  emit('pull', newModelName.value)
  newModelName.value = ''
}

/**
 * Checks if a model is currently running.
 * @param {string} name 
 */
const isModelRunning = (name) => {
  return props.runningModels.some(m => m.name === name)
}
</script>

<style scoped>
.mb-3 { margin-bottom: 16px; }
.mb-4 { margin-bottom: 24px; }
.gap-2 { gap: 8px; }
.d-flex { display: flex; align-items: center; }
.text-center { text-align: center; }
.p-3 { padding: 16px; }
.list-group { display: flex; flex-direction: column; }
</style>
