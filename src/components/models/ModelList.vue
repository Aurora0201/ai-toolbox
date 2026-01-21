<template>
  <div class="bg-background-surface border border-border rounded-lg shadow-sm mb-6 overflow-hidden">
    <div class="px-4 py-3 border-b border-border bg-background-element font-semibold text-sm flex justify-between items-center text-text-main">
      <span class="flex items-center gap-2">
        <Box class="w-4 h-4" /> {{ $t('models.listTitle') }}
      </span>
      <span class="text-text-sub font-mono text-xs">{{ models.length }} {{ $t('models.items') }}</span>
    </div>
    <div class="p-4">
      <!-- Pull Model Input -->
      <div class="flex gap-2 mb-4">
        <input 
          v-model="newModelName" 
          :placeholder="$t('models.pullPlaceholder')" 
          class="flex-1 w-full p-2.5 rounded-md border border-border bg-background-surface text-text-main text-sm outline-none focus:border-primary focus:ring-2 focus:ring-primary/20 transition-all appearance-none"
          @keyup.enter="handlePull"
        >
        <button
          :disabled="pulling"
          class="px-4 py-2 rounded-md font-medium text-sm flex items-center gap-2 transition-colors bg-primary text-white hover:bg-primary-hover disabled:opacity-50 disabled:cursor-not-allowed whitespace-nowrap"
          @click="handlePull"
        >
          <Download class="w-4 h-4" />
          {{ pulling ? $t('models.pulling') : $t('models.pull') }}
        </button>
      </div>

      <!-- Pull Progress Bar -->
      <div
        v-if="pulling"
        class="p-3 bg-background-element border border-border rounded-md mb-4"
      >
        <div class="flex justify-between text-xs text-text-sub mb-1">
          <span class="font-medium">{{ pullProgress.status }}</span>
          <span class="font-mono">{{ pullProgress.percentage }}%</span>
        </div>
        <div class="h-1.5 bg-border rounded-full overflow-hidden">
          <div 
            class="h-full bg-primary transition-all duration-300" 
            :style="{ width: pullProgress.percentage + '%' }"
          />
        </div>
      </div>
      
      <!-- Models List -->
      <div
        v-if="loading"
        class="text-center p-4 text-text-sub flex flex-col items-center gap-2"
      >
        <Loader2 class="w-6 h-6 text-primary animate-spin" />
        {{ $t('models.loading') }}
      </div>
      <div
        v-else
        class="flex flex-col"
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
import { Box, Download, Loader2 } from 'lucide-vue-next'

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
  pullProgress: {
    type: Object,
    default: () => ({ status: '', completed: 0, total: 0, percentage: 0 })
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

