<template>
  <div
    class="flex items-start p-3 bg-white dark:bg-slate-800 rounded-md shadow-md min-w-[300px] max-w-[400px] border-l-4 pointer-events-auto"
    :class="{
      'border-success': type === 'success',
      'border-danger': type === 'error',
      'border-warning': type === 'warning',
      'border-primary': type === 'info'
    }"
    role="alert"
  >
    <div class="mr-3 text-base flex items-center pt-0.5">
      <CheckCircle v-if="type === 'success'" class="w-4 h-4 text-success" />
      <AlertCircle v-else-if="type === 'error'" class="w-4 h-4 text-danger" />
      <AlertTriangle v-else-if="type === 'warning'" class="w-4 h-4 text-warning" />
      <Info v-else class="w-4 h-4 text-primary" />
    </div>
    <div class="flex-1 text-sm text-text-main leading-snug">
      {{ message }}
    </div>
    <button
      class="text-text-sub hover:text-text-main cursor-pointer bg-transparent border-none p-0 ml-2 flex items-center"
      @click="$emit('close')"
    >
      <X class="w-4 h-4" />
    </button>
  </div>
</template>

<script setup>
import { CheckCircle, AlertCircle, AlertTriangle, Info, X } from 'lucide-vue-next'

defineProps({
  message: { type: String, required: true },
  type: { type: String, default: 'info' } // success, error, warning, info
})

defineEmits(['close'])
</script>

