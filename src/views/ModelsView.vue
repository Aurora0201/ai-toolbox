<template>
  <div class="p-8 max-w-[1000px] mx-auto">
    <div class="mb-6">
      <h1 class="text-2xl font-bold text-text-main">{{ t.title }}</h1>
      <p class="text-text-sub mt-1">
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