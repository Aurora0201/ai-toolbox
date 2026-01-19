<template>
  <div class="flex items-center p-4 border-b border-border gap-4 last:border-0">
    <!-- Icon Column -->
    <div class="shrink-0">
      <div class="w-12 h-12 bg-background-element rounded-lg flex items-center justify-center text-primary shrink-0">
        <Box class="w-6 h-6" />
      </div>
    </div>

    <!-- Info Column (Name & Size) -->
    <div class="flex-1 min-w-0 flex flex-col justify-center gap-1">
      <div class="font-semibold text-sm text-text-main truncate font-mono">
        {{ model.name }}
      </div>
      <div class="text-xs text-text-sub leading-none">
        {{ formatSize(model.size) }}
      </div>
    </div>

    <!-- Actions Column -->
    <div class="shrink-0 flex items-center gap-2">
      <!-- Start/Running Status Button -->
      <button 
        class="px-3 py-1.5 rounded-md text-xs font-medium flex items-center gap-1 transition-colors"
        :class="isRunning ? 'bg-success text-white' : 'bg-primary text-white hover:bg-primary-hover disabled:opacity-50 disabled:cursor-not-allowed'" 
        :disabled="isRunning || loadingState === 'starting'"
        @click="$emit('start', model.name)"
      >
        <template v-if="loadingState === 'starting'">
          <Loader2 class="w-3 h-3 animate-spin" /> {{ t.starting }}
        </template>
        <template v-else-if="isRunning">
          <Check class="w-3 h-3" />
          {{ t.running }}
        </template>
        <template v-else>
          <Play class="w-3 h-3 fill-current" />
          {{ t.start }}
        </template>
      </button>
      
      <!-- Delete Button -->
      <button 
        class="px-3 py-1.5 rounded-md text-xs font-medium flex items-center gap-1 transition-colors bg-danger text-white hover:opacity-90 disabled:opacity-50" 
        :disabled="!!loadingState"
        @click="$emit('delete', model.name)"
      >
        <Loader2 v-if="loadingState === 'deleting'" class="w-3 h-3 animate-spin" />
        <Trash2 v-else class="w-3 h-3" />
        {{ loadingState === 'deleting' ? '...' : t.delete }}
      </button>
    </div>
  </div>
</template>

<script setup>
import { computed } from 'vue'
import { useSettingsStore } from '../../store/settings'
import { Box, Play, Check, Trash2, Loader2 } from 'lucide-vue-next'

/**
 * ModelItem component displays a single model's information and action buttons.
 */
defineProps({
  model: {
    type: Object,
    required: true
  },
  isRunning: {
    type: Boolean,
    default: false
  },
  loadingState: {
    type: String,
    default: null
  }
})

const settings = useSettingsStore()

const translations = {
  en: {
    starting: 'Starting...',
    running: 'Running',
    start: 'Start',
    delete: 'Delete'
  },
  zh: {
    starting: '启动中...',
    running: '运行中',
    start: '启动',
    delete: '删除'
  }
}

const t = computed(() => translations[settings.language] || translations.en)

defineEmits(['start', 'delete'])

/**
 * Formats byte size into human-readable string.
 * @param {number} bytes 
 */
const formatSize = (bytes) => {
  if (bytes === 0) return '0 B'
  const k = 1024
  const sizes = ['B', 'KB', 'MB', 'GB', 'TB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i]
}
</script>

