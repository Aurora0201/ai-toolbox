<template>
  <div class="bg-background-surface border border-border rounded-lg shadow-sm mt-6 overflow-hidden">
    <div class="px-4 py-3 border-b border-border bg-background-element font-semibold text-sm flex justify-between items-center text-text-main">
      <span>{{ $t('dashboard.chartTitle') }}</span>
      <div class="flex items-center text-xs text-text-sub">
        <span class="w-2 h-2 rounded-full mr-1 inline-block bg-primary" /> {{ $t('dashboard.input') }}
        <span class="w-2 h-2 rounded-full mr-1 inline-block bg-success ml-3" /> {{ $t('dashboard.output') }}
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
import { ref, onMounted, onUnmounted, watch } from 'vue'
import * as echarts from 'echarts'
import { dbApi } from '../../../api/db'
import { useI18n } from 'vue-i18n'

/**
 * TokenChart component visualizes input and output token usage over time using ECharts.
 */
const chartRef = ref(null)
const { t, locale } = useI18n()
let chart = null

/**
 * Fetches token statistics from the backend and updates the chart.
 */
const updateChart = async () => {
  try {
    const stats = await dbApi.getTokenStats()
    const dates = stats.map(s => s.date)
    const promptTokens = stats.map(s => s.prompt_tokens)
    const completionTokens = stats.map(s => s.completion_tokens)

    const option = {
      backgroundColor: 'transparent',
      tooltip: {
        trigger: 'axis',
        backgroundColor: 'rgba(255, 255, 255, 0.9)',
        borderColor: '#dee2e6',
        textStyle: { color: '#212529', fontFamily: '"Google Sans", "Roboto", "Noto Sans SC", sans-serif' },
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
        axisLabel: { color: '#6c757d', fontFamily: "'Roboto Mono', monospace" },
        axisTick: { show: false }
      },
      yAxis: {
        type: 'value',
        axisLine: { show: false },
        splitLine: { lineStyle: { color: '#f1f3f5' } },
        axisLabel: { color: '#6c757d', fontFamily: "'Roboto Mono', monospace" }
      },
      series: [
        {
          name: t('dashboard.input'),
          type: 'line',
          data: promptTokens,
          smooth: true,
          symbol: 'circle',
          symbolSize: 6,
          itemStyle: { color: '#2563EB' },
          lineStyle: { width: 2 }
        },
        {
          name: t('dashboard.output'),
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

watch(locale, () => {
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