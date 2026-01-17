<template>
  <div class="panel">
    <div
      class="panel-header"
      style="justify-content: space-between; align-items: center;"
    >
      <span class="d-flex gap-2">🚀 {{ t.title }}</span>
      
      <!-- GPU Info Section -->
      <div
        v-if="gpuInfo.name || gpuInfo.total > 0"
        class="d-flex flex-column align-items-end"
      >
        <div
          v-if="gpuInfo.name"
          class="text-muted text-mono mb-1"
          style="font-size: 10px;"
        >
          {{ gpuInfo.name }}
        </div>
        <div
          v-if="gpuInfo.total > 0"
          class="d-flex align-items-center gap-2 text-muted"
          style="font-size: 11px;"
        >
          <div
            class="vram-progress-track"
            style="width: 80px;"
          >
            <div 
              class="vram-progress-fill" 
              :style="{ 
                width: (gpuInfo.used / gpuInfo.total * 100) + '%', 
                backgroundColor: (gpuInfo.used / gpuInfo.total) > 0.8 ? 'var(--danger-color)' : 'var(--primary-color)' 
              }"
            />
          </div>
          <span>{{ (gpuInfo.used / gpuInfo.total * 100).toFixed(0) }}%</span>
        </div>
      </div>
      <span
        v-else
        class="text-success text-mono"
        style="font-size: 12px"
      >{{ t.active }}</span>
    </div>
    
    <div class="panel-body">
      <!-- Empty State -->
      <div
        v-if="runningModels.length === 0"
        class="text-center p-4 text-muted"
      >
        <div style="font-size: 24px; margin-bottom: 8px;">
          😴
        </div>
        {{ t.noModels }}
      </div>
      
      <!-- Running Models List -->
      <div
        v-else
        class="list-group"
      >
        <div
          v-for="model in runningModels"
          :key="model.name"
          class="list-item"
        >
          <!-- Left side: Icon & Name -->
          <div class="item-main-info">
            <div class="mini-icon-box">
              <svg
                xmlns="http://www.w3.org/2000/svg"
                width="16"
                height="16"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
              ><path d="M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z" /><polyline points="3.27 6.96 12 12.01 20.73 6.96" /><line
                x1="12"
                y1="22.08"
                x2="12"
                y2="12"
              /></svg>
            </div>
            <div class="item-name text-mono text-primary">
              {{ model.name }}
            </div>
            <div class="status-indicator">
              <span class="status-dot" />
              <span class="status-text text-success">{{ t.active }}</span>
            </div>
          </div>

          <!-- Right side: Actions -->
          <div class="item-actions">
            <button 
              class="btn btn-sm btn-danger d-flex gap-1" 
              :disabled="loadingStates[model.name] === 'stopping'"
              @click="$emit('stop', model.name)"
            >
              <template v-if="loadingStates[model.name] === 'stopping'">
                ⏳ {{ t.stopping }}
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
                ><rect
                  x="3"
                  y="3"
                  width="18"
                  height="18"
                  rx="2"
                  ry="2"
                /></svg>
                {{ t.stop }}
              </template>
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { computed } from 'vue'
import { useSettingsStore } from '../../store/settings'

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

const settings = useSettingsStore()

const translations = {
  en: {
    title: 'Running Processes',
    active: 'Active',
    noModels: 'No models currently running.',
    vram: 'VRAM',
    stop: 'Stop',
    stopping: 'Stopping...'
  },
  zh: {
    title: '运行中的进程',
    active: '活跃',
    noModels: '当前没有正在运行的模型。',
    vram: '显存占用',
    stop: '停止',
    stopping: '停止中...'
  }
}

const t = computed(() => translations[settings.language] || translations.en)

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

<style scoped>
.d-flex { display: flex; align-items: center; }
.flex-column { flex-direction: column; }
.align-items-end { align-items: flex-end; }
.gap-2 { gap: 8px; }
.gap-1 { gap: 4px; }
.mb-1 { margin-bottom: 4px; }
.text-center { text-align: center; }
.p-4 { padding: 32px; }

.list-group { display: flex; flex-direction: column; }

.list-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 16px 12px;
  border-bottom: 2px solid var(--border-color);
}

.list-item:last-child {
  border-bottom: none;
  padding-bottom: 0;
}

.item-main-info {
  display: flex;
  align-items: center;
  gap: 12px;
  flex: 1;
  min-width: 0;
}

.mini-icon-box {
  width: 32px;
  height: 32px;
  background-color: var(--bg-hover);
  border-radius: var(--radius-sm);
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--primary-color);
  flex-shrink: 0;
}

.item-name {
  font-weight: 600;
  font-size: 14px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.status-indicator {
  display: flex;
  align-items: center;
  gap: 6px;
  flex-shrink: 0;
}

.status-text {
  font-size: 12px;
  font-weight: 500;
}

.status-dot {
  width: 8px;
  height: 8px;
  background-color: var(--success-color);
  border-radius: 50%;
  display: inline-block;
  box-shadow: 0 0 0 2px rgba(25, 135, 84, 0.2);
  animation: pulse 2s infinite;
}

@keyframes pulse {
  0% { box-shadow: 0 0 0 0 rgba(25, 135, 84, 0.4); }
  70% { box-shadow: 0 0 0 6px rgba(25, 135, 84, 0); }
  100% { box-shadow: 0 0 0 0 rgba(25, 135, 84, 0); }
}

.vram-progress-track {
  height: 6px;
  background-color: var(--bg-hover);
  border-radius: 3px;
  overflow: hidden;
}

.vram-progress-fill {
  height: 100%;
  background-color: var(--primary-color);
  border-radius: 3px;
  transition: width 0.3s ease;
}
</style>
