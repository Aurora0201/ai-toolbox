<template>
  <div
    class="fixed z-[9999] p-5 pointer-events-none flex flex-col gap-2.5"
    :class="positionClasses[position]"
  >
    <TransitionGroup
      enter-active-class="transition-all duration-300 ease-out"
      enter-from-class="opacity-0 translate-x-full"
      leave-active-class="transition-all duration-300 ease-in absolute"
      leave-to-class="opacity-0 translate-x-full"
      move-class="transition-all duration-300 ease-out"
    >
      <ToastNotification
        v-for="toast in toasts"
        :key="toast.id"
        :message="toast.message"
        :type="toast.type"
        @close="removeToast(toast.id)"
      />
    </TransitionGroup>
  </div>
</template>

<script setup>
import { useToast } from '../../composables/useToast'
import ToastNotification from './ToastNotification.vue'

const props = defineProps({
  position: {
    type: String,
    default: 'top-right',
    validator: (value) => ['top-left', 'top-right', 'bottom-left', 'bottom-right'].includes(value)
  }
})

const positionClasses = {
  'top-right': 'top-10 right-0',
  'top-left': 'top-10 left-0',
  'bottom-right': 'bottom-0 right-0 flex-col-reverse',
  'bottom-left': 'bottom-0 left-0 flex-col-reverse'
}

const { toasts, removeToast } = useToast()
</script>

