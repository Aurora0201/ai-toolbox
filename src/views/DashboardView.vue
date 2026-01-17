<template>
  <div class="dashboard-view">
    <div class="header-section mb-4">
      <h1>{{ t.title }}</h1>
      <p class="text-muted">
        {{ t.subtitle }}
      </p>
    </div>

    <div class="stats-overview grid-3 mb-4">
      <div class="panel stat-card">
        <div class="panel-body">
          <div class="stat-label">
            {{ t.totalPrompt }}
          </div>
          <div class="stat-value text-primary">
            {{ totalPrompt }}
          </div>
        </div>
      </div>
      <div class="panel stat-card">
        <div class="panel-body">
          <div class="stat-label">
            {{ t.totalCompletion }}
          </div>
          <div class="stat-value text-success">
            {{ totalCompletion }}
          </div>
        </div>
      </div>
      <div class="panel stat-card">
        <div class="panel-body">
          <div class="stat-label">
            {{ t.activeDays }}
          </div>
          <div class="stat-value">
            {{ activeDays }}
          </div>
        </div>
      </div>
    </div>
    
    <TokenChart />
  </div>
</template>

<script setup>
/**
 * Dashboard view providing detailed usage analytics and statistics.
 */
import { ref, onMounted, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { useSettingsStore } from '../store/settings'
import TokenChart from '../components/TokenChart.vue'

const settings = useSettingsStore()
const totalPrompt = ref(0)
const totalCompletion = ref(0)
const activeDays = ref(0)

const translations = {
  en: {
    title: 'Usage Analytics',
    subtitle: 'Visualize your model interactions and token consumption over time.',
    totalPrompt: 'Total Prompt Tokens',
    totalCompletion: 'Total Completion Tokens',
    activeDays: 'Days Active'
  },
  zh: {
    title: '用量分析',
    subtitle: '可视化您的模型交互和随时间变化的 Token 消耗。',
    totalPrompt: '总提示词 Token',
    totalCompletion: '总生成词 Token',
    activeDays: '活跃天数'
  }
}

const t = computed(() => translations[settings.language] || translations.en)

/**
 * Fetches and calculates summary statistics from the token history.
 */
const fetchStats = async () => {
  try {
    const stats = await invoke('get_token_stats')
    totalPrompt.value = stats.reduce((acc, curr) => acc + curr.prompt_tokens, 0).toLocaleString()
    totalCompletion.value = stats.reduce((acc, curr) => acc + curr.completion_tokens, 0).toLocaleString()
    activeDays.value = stats.length
  } catch (error) {
    console.error('Failed to fetch stats:', error)
  }
}

onMounted(() => {
  fetchStats()
})
</script>

<style scoped>
.dashboard-view {
  padding: 32px;
  max-width: 1000px;
  margin: 0 auto;
}

.mb-4 { margin-bottom: 32px; }

.grid-3 {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 24px;
}

.stat-card {
  text-align: center;
}

.stat-label {
  font-size: 12px;
  font-weight: 600;
  color: var(--text-secondary);
  text-transform: uppercase;
  margin-bottom: 8px;
}

.stat-value {
  font-size: 28px;
  font-weight: 700;
  font-family: var(--font-mono);
}

@media (max-width: 768px) {
  .grid-3 {
    grid-template-columns: 1fr;
  }
}
</style>
