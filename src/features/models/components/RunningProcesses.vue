<template>
  <div class="bg-background-surface border border-border rounded-lg shadow-sm overflow-hidden">
    <div class="px-4 py-3 border-b border-border bg-background-element font-semibold text-sm flex justify-between items-center text-text-main">
      <span class="flex items-center gap-2">
        <Rocket class="w-4 h-4" /> {{ $t('models.processesTitle') }}
      </span>
      
      <!-- GPU Info Section -->
      <div
        v-if="gpuInfo.name || gpuInfo.total > 0"
        class="flex flex-col items-end"
      >
        <div
          v-if="gpuInfo.name"
          class="text-xs text-text-sub font-mono mb-1"
        >
          {{ gpuInfo.name }}
        </div>
        <div
          v-if="gpuInfo.total > 0"
          class="flex items-center gap-2 text-xs text-text-sub"
        >
          <div class="w-20 h-1.5 bg-background-element border border-border rounded-full overflow-hidden">
            <div 
              class="h-full transition-all duration-300" 
              :class="(gpuInfo.used / gpuInfo.total) > 0.8 ? 'bg-danger' : 'bg-primary'"
              :style="{ width: (gpuInfo.used / gpuInfo.total * 100) + '%' }"
            />
          </div>
          <span>{{ (gpuInfo.used / gpuInfo.total * 100).toFixed(0) }}%</span>
        </div>
      </div>
      <span
        v-else
        class="text-success text-mono text-xs"
      >{{ $t('models.active') }}</span>
    </div>
    
    <div class="p-4">
      <!-- Empty State -->
      <div
        v-if="runningModels.length === 0"
        class="text-center p-8 text-text-sub text-sm"
      >
        <div class="text-2xl mb-2">
          😴
        </div>
        {{ $t('models.noModels') }}
      </div>
      
      <!-- Running Models List -->
      <div
        v-else
        class="flex flex-col"
      >
        <div
          v-for="model in runningModels"
          :key="model.name"
          class="flex justify-between items-center p-4 border-b border-border last:border-0"
        >
          <!-- Left side: Icon & Name -->
          <div class="flex items-center gap-3 flex-1 min-w-0">
            <div class="w-8 h-8 bg-background-element rounded-md flex items-center justify-center text-primary shrink-0">
              <Box class="w-4 h-4" />
            </div>
            <div class="font-semibold text-sm font-mono text-primary truncate">
              {{ model.name }}
            </div>
            <div class="flex items-center gap-1.5 shrink-0">
              <span class="w-2 h-2 bg-success rounded-full animate-pulse shadow-[0_0_0_2px_rgba(16,185,129,0.2)]" />
              <span class="text-xs font-medium text-success">{{ $t('models.active') }}</span>
            </div>
          </div>

          <!-- Right side: Actions -->
          <div class="shrink-0">
            <button 
              class="px-3 py-1.5 rounded-md text-xs font-medium flex items-center gap-1 transition-colors bg-danger text-white hover:opacity-90 disabled:opacity-50 disabled:cursor-not-allowed"
              :disabled="!!loadingStates[model.name]"
              @click="$emit('stop', model.name)"
            >
              <template v-if="loadingStates[model.name] === 'stopping'">
                <Loader2 class="w-3 h-3 animate-spin" /> {{ $t('models.stopping') }}
              </template>
              <template v-else>
                <Square class="w-3 h-3 fill-current" />
                {{ $t('models.stop') }}
              </template>
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { Rocket, Box, Square, Loader2 } from 'lucide-vue-next'

/**
 * RunningProcesses component shows currently active models and system resource usage.
 */
defineProps({
  runningModels: {
    type: Array,
    required: true
  },
  gpuInfo: {
    type: Object,
    required: true
  },
  loadingStates: {
    type: Object,
    default: () => ({})
  }
})

defineEmits(['stop'])

/**
 * Formats byte size into human-readable string.
 */
const formatSize = (bytes) => {
  if (bytes === 0) return '0 B'
  const k = 1024
  const sizes = ['B', 'KB', 'MB', 'GB', 'TB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i]
}
</script>

