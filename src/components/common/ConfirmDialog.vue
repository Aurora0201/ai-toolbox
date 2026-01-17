<template>
  <Transition name="fade">
    <div
      v-if="show"
      class="confirm-overlay"
      @click.self="cancel"
    >
      <div class="confirm-dialog panel">
        <div class="panel-header">
          <span>{{ title }}</span>
          <button
            class="close-btn"
            @click="cancel"
          >
            &times;
          </button>
        </div>
        <div class="panel-body">
          <p class="confirm-message">
            {{ message }}
          </p>
          <div class="confirm-actions">
            <button
              class="btn btn-outline"
              @click="cancel"
            >
              {{ cancelText }}
            </button>
            <button
              class="btn btn-danger"
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

<style scoped>
.confirm-overlay {
  position: fixed;
  top: 0;
  left: 0;
  width: 100vw;
  height: 100vh;
  background-color: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 9999;
}

.confirm-dialog {
  width: 100%;
  max-width: 400px;
  background-color: var(--bg-panel);
  box-shadow: var(--shadow-md);
  animation: slide-up 0.2s ease-out;
}

.confirm-message {
  margin-bottom: 24px;
  font-size: 14px;
  color: var(--text-primary);
  line-height: 1.5;
}

.confirm-actions {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
}

.close-btn {
  background: none;
  border: none;
  color: var(--text-secondary);
  font-size: 20px;
  cursor: pointer;
  padding: 0;
  line-height: 1;
}

.close-btn:hover {
  color: var(--text-primary);
}

@keyframes slide-up {
  from {
    transform: translateY(20px);
    opacity: 0;
  }
  to {
    transform: translateY(0);
    opacity: 1;
  }
}

.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.2s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}
</style>
