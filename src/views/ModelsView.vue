<template>
  <div class="p-8 max-w-[1000px] mx-auto">
    <div class="mb-6">
      <h1 class="text-2xl font-bold text-text-main">
        {{ $t('models.title') }}
      </h1>
      <p class="text-text-sub mt-1">
        {{ $t('models.subtitle') }}
      </p>
    </div>
    
    <ModelManager />
  </div>
</template>

<script setup>
/**
 * Models view dedicated to managing the local Ollama model library.
 */
import ModelManager from '../components/ModelManager.vue'
</script>