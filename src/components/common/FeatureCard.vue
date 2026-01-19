<template>
  <div
    class="bg-background-surface border border-border rounded-lg shadow-sm h-full cursor-pointer transition-all duration-300 hover:-translate-y-1 hover:border-primary hover:shadow-md group"
    @click="$emit('click')"
  >
    <div class="flex items-center gap-5 p-6">
      <div class="w-14 h-14 bg-background-element rounded-lg flex items-center justify-center text-3xl shrink-0">
        <slot name="icon" />
      </div>
      <div class="flex-1">
        <h3 class="text-lg font-semibold text-text-main mb-1">
          {{ title }}
        </h3>
        <p class="text-sm text-text-sub leading-snug">
          {{ description }}
        </p>
      </div>
      <div class="text-text-sub transition-transform duration-200 group-hover:text-primary group-hover:translate-x-1">
        <ArrowRight class="w-5 h-5" />
      </div>
    </div>
  </div>
</template>

<script setup>
import { ArrowRight } from 'lucide-vue-next'

/**
 * Common component for feature cards displayed on the home page.
 */
defineProps({
  title: {
    type: String,
    required: true
  },
  description: {
    type: String,
    required: true
  }
})

defineEmits(['click'])
</script>

