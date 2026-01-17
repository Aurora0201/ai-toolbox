<template>
  <div
    :class="['toast', type]"
    role="alert"
  >
    <div class="toast-icon">
      <span v-if="type === 'success'">✅</span>
      <span v-else-if="type === 'error'">❌</span>
      <span v-else-if="type === 'warning'">⚠️</span>
      <span v-else>ℹ️</span>
    </div>
    <div class="toast-content">
      {{ message }}
    </div>
    <button
      class="toast-close"
      @click="$emit('close')"
    >
      ×
    </button>
  </div>
</template>

<script setup>
defineProps({
  message: { type: String, required: true },
  type: { type: String, default: 'info' } // success, error, warning, info
})

defineEmits(['close'])
</script>

<style scoped>
.toast {
  display: flex;
  align-items: flex-start;
  padding: 12px 16px;
  background: white;
  border-radius: var(--radius-md);
  box-shadow: var(--shadow-md);
  min-width: 300px;
  max-width: 400px;
  border-left: 4px solid transparent;
  pointer-events: auto;
}

.toast.success { border-left-color: var(--success-color); }
.toast.error { border-left-color: var(--danger-color); }
.toast.warning { border-left-color: var(--warning-color); }
.toast.info { border-left-color: var(--primary-color); }

.toast-icon {
  margin-right: 12px;
  font-size: 16px;
}

.toast-content {
  flex: 1;
  font-size: 14px;
  color: var(--text-primary);
  line-height: 1.4;
}

.toast-close {
  background: none;
  border: none;
  color: var(--text-secondary);
  font-size: 20px;
  line-height: 1;
  cursor: pointer;
  padding: 0 0 0 8px;
  margin-top: -2px;
}

.toast-close:hover {
  color: var(--text-primary);
}
</style>
