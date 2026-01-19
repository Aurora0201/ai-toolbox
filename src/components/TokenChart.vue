<template>
  <div class="bg-background-surface border border-border rounded-lg shadow-sm mt-6">
    <div class="px-4 py-3 border-b border-border bg-background-element font-semibold text-sm flex justify-between items-center text-text-main">
      <span>{{ t.title }}</span>
      <div class="flex items-center text-xs text-text-sub">
        <span class="w-2 h-2 rounded-full mr-1 inline-block bg-primary" /> {{ t.input }}
        <span class="w-2 h-2 rounded-full mr-1 inline-block bg-success ml-3" /> {{ t.output }}
      </div>
    </div>
    <div class="p-4">
      <div
        ref="chartRef"
        class="h-[300px] w-full"
      />
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted, onUnmounted, computed, watch } from 'vue'
import * as echarts from 'echarts'
import { invoke } from '@tauri-apps/api/core'
import { useSettingsStore } from '../store/settings'

/**
 * TokenChart component visualizes input and output token usage over time using ECharts.
 */
const chartRef = ref(null)
const settings = useSettingsStore()
let chart = null

const translations = {
  en: {
    title: 'Token Usage Trend',
    input: 'Input',
    output: 'Output'
  },
  zh: {
    title: 'Token 使用趋势',
    input: '提示词 (Input)',
    output: '生成词 (Output)'
  }
}

const t = computed(() => translations[settings.language] || translations.en)

/**
 * Fetches token statistics from the backend and updates the chart.
 */
const updateChart = async () => {
  try {
    const stats = await invoke('get_token_stats')
    const dates = stats.map(s => s.date)
    const promptTokens = stats.map(s => s.prompt_tokens)
    const completionTokens = stats.map(s => s.completion_tokens)

    const option = {
      backgroundColor: 'transparent',
      tooltip: {
        trigger: 'axis',
        backgroundColor: 'rgba(255, 255, 255, 0.9)',
        borderColor: '#dee2e6',
        textStyle: { color: '#212529', fontFamily: 'sans-serif' },
        axisPointer: { type: 'line', lineStyle: { color: '#adb5bd' } }
      },
      grid: {
        left: '20px',
        right: '20px',
        bottom: '10px',
        top: '10px',
        containLabel: true
      },
      xAxis: {
        type: 'category',
        data: dates,
        axisLine: { lineStyle: { color: '#dee2e6' } },
        axisLabel: { color: '#6c757d', fontFamily: 'monospace' },
        axisTick: { show: false }
      },
      yAxis: {
        type: 'value',
        axisLine: { show: false },
        splitLine: { lineStyle: { color: '#f1f3f5' } },
        axisLabel: { color: '#6c757d', fontFamily: 'monospace' }
      },
      series: [
        {
          name: t.value.input,
          type: 'line',
          data: promptTokens,
          smooth: true,
          symbol: 'circle',
          symbolSize: 6,
          itemStyle: { color: '#2563EB' },
          lineStyle: { width: 2 }
        },
        {
          name: t.value.output,
          type: 'line',
          data: completionTokens,
          smooth: true,
          symbol: 'circle',
          symbolSize: 6,
          itemStyle: { color: '#10B981' },
          lineStyle: { width: 2 }
        }
      ]
    }
    chart.setOption(option)
  } catch (error) {
    console.error('Failed to update chart:', error)
  }
}

watch(() => settings.language, () => {
  updateChart()
})

onMounted(() => {
  // Initialize chart instance
  chart = echarts.init(chartRef.value)
  updateChart()
  
  // Handle window resizing
  window.addEventListener('resize', () => chart && chart.resize())
})

onUnmounted(() => {
  // Clean up chart instance
  if (chart) {
    chart.dispose()
  }
})
</script>