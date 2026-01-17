<template>
  <div
    class="toast-container"
    :class="position"
  >
    <TransitionGroup name="toast">
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
import { useToast } from '../composables/useToast'
import ToastNotification from './ToastNotification.vue'

const props = defineProps({
  position: {
    type: String,
    default: 'top-right',
    validator: (value) => ['top-left', 'top-right', 'bottom-left', 'bottom-right'].includes(value)
  }
})

const { toasts, removeToast } = useToast()
</script>

<style scoped>
.toast-container {
  position: fixed;
  z-index: 9999;
  padding: 20px;
  pointer-events: none;
  display: flex;
  flex-direction: column;
  gap: 10px;
}

/* Positioning - Adjusted to avoid Title Bar (approx 32px + padding) */
.top-right { top: 40px; right: 0; }
.top-left { top: 40px; left: 0; }
.bottom-right { bottom: 0; right: 0; }
.bottom-left { bottom: 0; left: 0; }

/* Stacking order */
.bottom-right, .bottom-left {
  flex-direction: column-reverse;
}

/* Vue Transitions */
.toast-enter-active,
.toast-leave-active {
  transition: all 0.4s ease;
}

.toast-enter-from {
  opacity: 0;
  transform: translateX(100%);
}

.toast-leave-to {
  opacity: 0;
  transform: translateX(100%);
}
</style>
