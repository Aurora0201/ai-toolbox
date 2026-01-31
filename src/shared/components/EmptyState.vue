<template>
  <div 
    ref="containerRef"
    class="flex-1 flex flex-col items-center justify-center opacity-0 select-none"
  >
    <div class="relative mb-6 group cursor-default">
      <div class="absolute inset-0 bg-primary/20 blur-xl rounded-full opacity-0 group-hover:opacity-100 transition-opacity duration-500 scale-150" />
      <div class="relative w-20 h-20 bg-background-surface border border-border rounded-2xl flex items-center justify-center shadow-lg shadow-primary/5 group-hover:scale-110 group-hover:-rotate-3 transition-transform duration-500 ease-out">
        <component 
          :is="icon" 
          class="w-10 h-10 text-primary group-hover:scale-110 transition-transform duration-300" 
        />
      </div>
    </div>
    
    <h2 class="empty-title text-2xl font-black uppercase tracking-[0.2em] text-transparent bg-clip-text bg-gradient-to-br from-text-main to-text-sub mb-3 translate-y-4 opacity-0">
      {{ title }}
    </h2>
    
    <p class="empty-desc text-sm font-medium text-text-sub/60 max-w-xs text-center leading-relaxed translate-y-4 opacity-0">
      {{ description }}
    </p>
  </div>
</template>

<script setup>
import { onMounted, ref } from 'vue'
import gsap from 'gsap'

const props = defineProps({
  icon: {
    type: Object,
    required: true
  },
  title: {
    type: String,
    required: true
  },
  description: {
    type: String,
    required: true
  }
})

const containerRef = ref(null)

onMounted(() => {
  const el = containerRef.value
  const titleEl = el.querySelector('.empty-title')
  const descEl = el.querySelector('.empty-desc')
  
  const tl = gsap.timeline({ defaults: { ease: 'power3.out' } })
  
  tl.to(el, {
    opacity: 1,
    duration: 0.5
  })
  .to(titleEl, {
    y: 0,
    opacity: 1,
    duration: 0.8
  }, '-=0.3')
  .to(descEl, {
    y: 0,
    opacity: 1,
    duration: 0.8
  }, '-=0.6')
})
</script>