<template>
  <div class="p-8 max-w-[1000px] mx-auto">
    <div class="mb-8">
      <h1 class="text-2xl font-bold text-text-main">
        {{ $t('dashboard.title') }}
      </h1>
      <p class="text-text-sub mt-1">
        {{ $t('dashboard.subtitle') }}
      </p>
    </div>

    <div class="grid grid-cols-1 md:grid-cols-3 gap-6 mb-8">
      <div class="bg-background-surface border border-border rounded-lg shadow-sm text-center">
        <div class="p-6">
          <div class="text-xs font-semibold text-text-sub uppercase mb-2">
            {{ $t('dashboard.totalPrompt') }}
          </div>
          <div class="text-3xl font-bold font-mono text-primary">
            {{ totalPrompt }}
          </div>
        </div>
      </div>
      <div class="bg-background-surface border border-border rounded-lg shadow-sm text-center">
        <div class="p-6">
          <div class="text-xs font-semibold text-text-sub uppercase mb-2">
            {{ $t('dashboard.totalCompletion') }}
          </div>
          <div class="text-3xl font-bold font-mono text-success">
            {{ totalCompletion }}
          </div>
        </div>
      </div>
      <div class="bg-background-surface border border-border rounded-lg shadow-sm text-center">
        <div class="p-6">
          <div class="text-xs font-semibold text-text-sub uppercase mb-2">
            {{ $t('dashboard.activeDays') }}
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
import { ref, onMounted } from 'vue'
import { dbApi } from '../../api/db'
import TokenChart from './components/TokenChart.vue'

const totalPrompt = ref(0)
const totalCompletion = ref(0)
const activeDays = ref(0)

/**
 * Fetches and calculates summary statistics from the token history.
 */
const fetchStats = async () => {
  try {
    const stats = await dbApi.getTokenStats()
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

