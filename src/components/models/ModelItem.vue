<template>
  <div class="list-item">
    <!-- Icon Column -->
    <div class="item-icon-col">
      <div class="icon-box">
        <svg
          xmlns="http://www.w3.org/2000/svg"
          width="24"
          height="24"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="1.5"
          stroke-linecap="round"
          stroke-linejoin="round"
        ><path d="M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z" /><polyline points="3.27 6.96 12 12.01 20.73 6.96" /><line
          x1="12"
          y1="22.08"
          x2="12"
          y2="12"
        /></svg>
      </div>
    </div>

    <!-- Info Column (Name & Size) -->
    <div class="item-info">
      <div class="item-name text-mono">
        {{ model.name }}
      </div>
      <div class="item-meta text-muted">
        {{ formatSize(model.size) }}
      </div>
    </div>

    <!-- Actions Column -->
    <div class="item-actions">
      <!-- Start/Running Status Button -->
      <button 
        class="btn btn-sm btn-primary mr-2 d-flex gap-1" 
        :class="{ 'btn-success': isRunning }" 
        :disabled="isRunning || loadingState === 'starting'"
        @click="$emit('start', model.name)"
      >
        <template v-if="loadingState === 'starting'">
          ⏳ {{ t.starting }}
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
          {{ t.running }}
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
          {{ t.start }}
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
        {{ loadingState === 'deleting' ? '...' : t.delete }}
      </button>
    </div>
  </div>
</template>

<script setup>
import { computed } from 'vue'
import { useSettingsStore } from '../../store/settings'

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

<style scoped>
.list-item {
  display: flex;
  align-items: center;
  padding: 16px 16px; /* Increased padding for spaciousness */
  border-bottom: 2px solid var(--border-color);
  gap: 16px;
}

.list-item:last-child {
  border-bottom: none;
  padding-bottom: 0;
}

.list-item:first-child {
  padding-top: 0;
}

/* Icon Column Styles */
.item-icon-col {
  flex-shrink: 0;
}

.icon-box {
  width: 48px;
  height: 48px;
  background-color: var(--bg-hover);
  border-radius: var(--radius-md); /* Smooth rounded icon container */
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--primary-color);
}

/* Info Column Styles */
.item-info {
  flex: 1;
  min-width: 0;
  display: flex;
  flex-direction: column;
  justify-content: center;
  gap: 4px; /* Space between name and size */
}

.item-name {
  font-weight: 600;
  font-size: 15px;
  color: var(--text-primary);
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
  line-height: 1.2;
}

.item-meta {
  font-size: 13px;
  color: var(--text-secondary);
  line-height: 1;
}

/* Actions Column Styles */
.item-actions {
  flex-shrink: 0;
  display: flex;
  align-items: center;
}

.mr-2 { margin-right: 8px; }
.d-flex { display: flex; align-items: center; }
.gap-1 { gap: 4px; }
</style>
