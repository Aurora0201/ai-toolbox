<template>
  <div class="models-view">
    <div class="header-section mb-4">
      <h1>{{ t.title }}</h1>
      <p class="text-muted">
        {{ t.subtitle }}
      </p>
    </div>
    
    <ModelManager />
  </div>
</template>

<script setup>
/**
 * Models view dedicated to managing the local Ollama model library.
 */
import { computed } from 'vue'
import { useSettingsStore } from '../store/settings'
import ModelManager from '../components/ModelManager.vue'

const settings = useSettingsStore()

const translations = {
  en: {
    title: 'Models',
    subtitle: 'Manage your local LLMs and monitor system resources.'
  },
  zh: {
    title: '模型',
    subtitle: '管理您的本地大语言模型并监控系统资源。'
  }
}

const t = computed(() => translations[settings.language] || translations.en)
</script>

<style scoped>
.models-view {
  padding: 32px;
  max-width: 1000px;
  margin: 0 auto;
}

.mb-4 { margin-bottom: 24px; }
</style>