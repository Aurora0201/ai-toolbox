<template>
  <div class="models-view">
    <div class="header-section mb-4">
      <h1>Models</h1>
      <p class="text-muted">
        Manage your local LLMs and monitor system resources.
      </p>
    </div>
    
    <ModelManager />
    <TokenChart />
  </div>
</template>

<script setup>
import ModelManager from '../components/ModelManager.vue'
import TokenChart from '../components/TokenChart.vue'
</script>

<style scoped>
.models-view {
  padding: 32px;
  max-width: 1000px;
  margin: 0 auto;
}

.mb-4 { margin-bottom: 24px; }
</style>
