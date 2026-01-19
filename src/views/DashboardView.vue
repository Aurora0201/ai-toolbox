<template>
  <div class="p-8 max-w-[1000px] mx-auto">
    <div class="mb-8">
      <h1 class="text-2xl font-bold text-text-main">{{ t.title }}</h1>
      <p class="text-text-sub mt-1">
        {{ t.subtitle }}
      </p>
    </div>

    <div class="grid grid-cols-1 md:grid-cols-3 gap-6 mb-8">
      <div class="bg-background-surface border border-border rounded-lg shadow-sm text-center">
        <div class="p-6">
          <div class="text-xs font-semibold text-text-sub uppercase mb-2">
            {{ t.totalPrompt }}
          </div>
          <div class="text-3xl font-bold font-mono text-primary">
            {{ totalPrompt }}
          </div>
        </div>
      </div>
      <div class="bg-background-surface border border-border rounded-lg shadow-sm text-center">
        <div class="p-6">
          <div class="text-xs font-semibold text-text-sub uppercase mb-2">
            {{ t.totalCompletion }}
          </div>
          <div class="text-3xl font-bold font-mono text-success">
            {{ totalCompletion }}
          </div>
        </div>
      </div>
      <div class="bg-background-surface border border-border rounded-lg shadow-sm text-center">
        <div class="p-6">
          <div class="text-xs font-semibold text-text-sub uppercase mb-2">
            {{ t.activeDays }}
          </div>
          <div class="text-3xl font-bold font-mono text-text-main">
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

