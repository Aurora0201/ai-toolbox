<template>
  <div class="list-item">
    <div class="item-info">
      <div class="item-name text-mono">
        <svg
          xmlns="http://www.w3.org/2000/svg"
          width="14"
          height="14"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
          stroke-linecap="round"
          stroke-linejoin="round"
          class="text-muted mr-1"
          style="vertical-align: middle;"
        ><path d="M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z" /><polyline points="3.27 6.96 12 12.01 20.73 6.96" /><line
          x1="12"
          y1="22.08"
          x2="12"
          y2="12"
        /></svg>
        {{ model.name }}
      </div>
      <div class="item-meta text-muted">
        {{ formatSize(model.size) }}
      </div>
    </div>
    <div class="item-actions">
      <!-- Start/Running Status Button -->
      <button 
        class="btn btn-sm btn-primary mr-2 d-flex gap-1" 
        :class="{ 'btn-success': isRunning }" 
        :disabled="isRunning || loadingState === 'starting'"
        @click="$emit('start', model.name)"
      >
        <template v-if="loadingState === 'starting'">
          ⏳ Starting...
        </template>
        <template v-else-if="isRunning">
          <svg
            xmlns="http://www.w3.org/2000/svg"
            width="12"
            height="12"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
          ><polyline points="20 6 9 17 4 12" /></svg>
          Running
        </template>
        <template v-else>
          <svg
            xmlns="http://www.w3.org/2000/svg"
            width="12"
            height="12"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
          ><polygon points="5 3 19 12 5 21 5 3" /></svg>
          Start
        </template>
      </button>
      
      <!-- Delete Button -->
      <button 
        class="btn btn-sm btn-danger d-flex gap-1" 
        :disabled="!!loadingState"
        @click="$emit('delete', model.name)"
      >
        <svg
          xmlns="http://www.w3.org/2000/svg"
          width="12"
          height="12"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
          stroke-linecap="round"
          stroke-linejoin="round"
        ><polyline points="3 6 5 6 21 6" /><path d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2" /></svg>
        {{ loadingState === 'deleting' ? '...' : 'Delete' }}
      </button>
    </div>
  </div>
</template>

<script setup>
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

<style scoped>
.list-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px 0;
  border-bottom: 1px solid var(--border-color);
}

.list-item:last-child {
  border-bottom: none;
  padding-bottom: 0;
}

.list-item:first-child {
  padding-top: 0;
}

.item-info {
  flex: 1;
  min-width: 0;
  margin-right: 16px;
}

.item-name {
  font-weight: 500;
  font-size: 14px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.item-meta {
  font-size: 12px;
  margin-top: 2px;
  margin-left: 20px;
}

.item-actions {
  flex-shrink: 0;
  display: flex;
  align-items: center;
}

.mr-2 { margin-right: 8px; }
.mr-1 { margin-right: 4px; }
.d-flex { display: flex; align-items: center; }
.gap-1 { gap: 4px; }
</style>
