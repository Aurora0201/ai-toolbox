<template>
  <div class="panel mt-4">
    <div class="panel-header">
      <span>Token Usage Trend</span>
      <div class="legend">
        <span class="dot primary" /> Input
        <span class="dot success ml-2" /> Output
      </div>
    </div>
    <div class="panel-body">
      <div
        ref="chartRef"
        class="chart"
      />
    </div>
  </div>
</template>

<script setup>
import { ref, onMounted, onUnmounted } from 'vue'
import * as echarts from 'echarts'
import { invoke } from '@tauri-apps/api/core'

/**
 * TokenChart component visualizes input and output token usage over time using ECharts.
 */
const chartRef = ref(null)
let chart = null

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
        textStyle: { color: '#212529', fontFamily: 'Inter' },
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
        axisLabel: { color: '#6c757d', fontFamily: 'JetBrains Mono' },
        axisTick: { show: false }
      },
      yAxis: {
        type: 'value',
        axisLine: { show: false },
        splitLine: { lineStyle: { color: '#f1f3f5' } },
        axisLabel: { color: '#6c757d', fontFamily: 'JetBrains Mono' }
      },
      series: [
        {
          name: 'Input',
          type: 'line',
          data: promptTokens,
          smooth: true,
          symbol: 'circle',
          symbolSize: 6,
          itemStyle: { color: '#0d6efd' },
          lineStyle: { width: 2 }
        },
        {
          name: 'Output',
          type: 'line',
          data: completionTokens,
          smooth: true,
          symbol: 'circle',
          symbolSize: 6,
          itemStyle: { color: '#198754' },
          lineStyle: { width: 2 }
        }
      ]
    }
    chart.setOption(option)
  } catch (error) {
    console.error('Failed to update chart:', error)
  }
}

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

<style scoped>
.mt-4 { margin-top: 24px; }
.chart { height: 300px; width: 100%; }

.legend {
  font-size: 12px;
  color: var(--text-secondary);
  display: flex;
  align-items: center;
}

.dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  display: inline-block;
  margin-right: 4px;
}
.dot.primary { background-color: var(--primary-color); }
.dot.success { background-color: var(--success-color); }
.ml-2 { margin-left: 12px; }
</style>