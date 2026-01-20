<template>
  <Transition
    enter-active-class="transition-opacity duration-200 ease-out"
    enter-from-class="opacity-0"
    leave-active-class="transition-opacity duration-200 ease-in"
    leave-to-class="opacity-0"
  >
    <div
      v-if="show"
      class="fixed top-0 left-0 w-screen h-screen bg-black/50 flex items-center justify-center z-[9999]"
      @click.self="cancel"
    >
      <div class="w-full max-w-[400px] bg-background-surface border border-border rounded-lg shadow-md animate-[slide-up_0.2s_ease-out] overflow-hidden">
        <div class="px-4 py-3 border-b border-border bg-background-element font-semibold text-sm flex justify-between items-center text-text-main">
          <span>{{ title }}</span>
          <button
            class="text-text-sub hover:text-text-main cursor-pointer bg-transparent border-none flex items-center justify-center p-0"
            @click="cancel"
          >
            <X class="w-4 h-4" />
          </button>
        </div>
        <div class="p-6">
          <p class="mb-6 text-sm text-text-main leading-relaxed">
            {{ message }}
          </p>
          <div class="flex justify-end gap-3">
            <button
              class="px-4 py-2 rounded-md font-medium text-sm transition-colors border border-border text-text-main hover:bg-background-element"
              @click="cancel"
            >
              {{ cancelText }}
            </button>
            <button
              class="px-4 py-2 rounded-md font-medium text-sm transition-colors bg-danger text-white hover:opacity-90"
              @click="confirm"
            >
              {{ confirmText }}
            </button>
          </div>
        </div>
      </div>
    </div>
  </Transition>
</template>

<script setup>
/**
 * Generic confirmation dialog component.
 * Adheres to OCP by allowing customization of text and actions.
 */
import { X } from 'lucide-vue-next'

defineProps({
  show: Boolean,
  title: {
    type: String,
    default: 'Confirm Action'
  },
  message: {
    type: String,
    default: 'Are you sure you want to proceed?'
  },
  confirmText: {
    type: String,
    default: 'Confirm'
  },
  cancelText: {
    type: String,
    default: 'Cancel'
  }
})

const emit = defineEmits(['confirm', 'cancel'])

const confirm = () => emit('confirm')
const cancel = () => emit('cancel')
</script>

