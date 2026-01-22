<template>
  <div class="min-h-full w-full bg-background-app relative overflow-hidden flex flex-col">
    <BackgroundEffect />

    <!-- Hero Section -->
    <header class="relative z-10 w-full max-w-[1200px] mx-auto pt-24 pb-16 px-8 flex flex-col items-center justify-center min-h-[50vh]">
      <!-- Animated Badge -->
      <div class="hero-badge opacity-0 translate-y-4 mb-10 px-4 py-1.5 rounded-full bg-background-surface border border-primary/30 flex items-center gap-2 shadow-[0_0_15px_rgba(var(--primary-rgb),0.3)] backdrop-blur-md">
        <span class="relative flex h-2.5 w-2.5">
          <span class="animate-ping absolute inline-flex h-full w-full rounded-full bg-primary opacity-75" />
          <span class="relative inline-flex rounded-full h-2.5 w-2.5 bg-primary" />
        </span>
        <span class="text-xs font-bold tracking-widest text-primary uppercase">
          AI Toolbox v0.1
        </span>
      </div>

      <!-- Animated Title Section -->
      <div class="mb-10 h-[1.3em] w-full flex items-center justify-center relative perspective-1000 overflow-visible">
        <!-- Text Container for Centering -->
        <h1 
          ref="titleRef"
          class="text-7xl md:text-8xl font-black tracking-tighter drop-shadow-2xl text-center whitespace-nowrap opacity-0 transform-style-3d text-transparent bg-clip-text bg-gradient-to-br from-text-main via-text-main/90 to-text-main/50 dark:from-white dark:via-white/90 dark:to-white/50 px-4 py-2"
        >
          <!-- Default content to prevent layout shift before JS loads -->
          AI Toolbox
        </h1>
      </div>

      <!-- Slogan -->
      <p class="hero-subtitle text-xl md:text-2xl text-text-sub max-w-3xl text-center font-light leading-relaxed opacity-0 translate-y-4 px-4">
        {{ $t('home.slogan') }}
      </p>

      <!-- CTA Buttons -->
      <div class="hero-actions flex gap-6 mt-12 opacity-0 translate-y-4">
        <button 
          class="group relative px-8 py-3.5 bg-primary text-white text-sm font-bold tracking-wide rounded-lg overflow-hidden shadow-lg shadow-primary/25 hover:shadow-primary/40 transition-all hover:scale-105 active:scale-95"
          @click="router.push('/chat')"
        >
          <div class="absolute inset-0 bg-white/20 translate-y-full group-hover:translate-y-0 transition-transform duration-300 ease-out" />
          <span class="relative flex items-center gap-2">
            {{ $t('home.startChat') }}
            <ArrowRight class="w-4 h-4 group-hover:translate-x-1 transition-transform" />
          </span>
        </button>
        <button 
          class="group px-8 py-3.5 bg-background-surface border border-border text-text-main text-sm font-bold tracking-wide rounded-lg hover:bg-background-element hover:border-primary/50 transition-all hover:scale-105 active:scale-95 flex items-center gap-2"
          @click="router.push('/models')"
        >
          {{ $t('home.manageModels') }}
        </button>
      </div>
    </header>

    <!-- Feature Cards Grid -->
    <main class="relative z-10 w-full max-w-[1200px] mx-auto px-8 pb-20">
      <div class="flex items-center gap-4 mb-8 opacity-50">
        <div class="h-px bg-gradient-to-r from-transparent via-border to-transparent flex-1" />
        <span class="text-xs font-mono uppercase tracking-[0.3em] text-text-sub">{{ $t('home.exploreFeatures') }}</span>
        <div class="h-px bg-gradient-to-r from-transparent via-border to-transparent flex-1" />
      </div>

      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
        <div 
          v-for="(feature, index) in features" 
          :key="index"
          class="feature-card opacity-0 translate-y-8"
          @click="router.push(feature.path)"
        >
          <div class="group h-full p-6 bg-background-surface/50 backdrop-blur-sm border border-border rounded-xl cursor-pointer transition-all duration-300 hover:bg-background-surface hover:border-primary/50 hover:shadow-[0_8px_30px_rgb(0,0,0,0.06)] hover:-translate-y-1 relative overflow-hidden">
            <!-- Hover Gradient Background -->
            <div class="absolute inset-0 bg-gradient-to-br from-primary/5 via-transparent to-transparent opacity-0 group-hover:opacity-100 transition-opacity duration-500" />
            
            <!-- Icon -->
            <div class="relative w-12 h-12 rounded-lg bg-background-element flex items-center justify-center mb-4 group-hover:scale-110 group-hover:bg-primary/10 transition-all duration-300">
              <component
                :is="feature.icon"
                class="w-6 h-6 text-text-sub group-hover:text-primary transition-colors"
              />
            </div>

            <!-- Text -->
            <h3 class="relative text-lg font-bold text-text-main mb-2 group-hover:text-primary transition-colors">
              {{ feature.title }}
            </h3>
            <p class="relative text-sm text-text-sub leading-relaxed group-hover:text-text-main/80 transition-colors">
              {{ feature.description }}
            </p>

            <!-- Arrow -->
            <div class="absolute bottom-4 right-4 opacity-0 -translate-x-2 group-hover:opacity-100 group-hover:translate-x-0 transition-all duration-300">
              <ArrowUpRight class="w-5 h-5 text-primary" />
            </div>
          </div>
        </div>
      </div>
    </main>
  </div>
</template>

<script setup>
import { onMounted, ref, nextTick } from 'vue'
import { useRouter } from 'vue-router'
import { useI18n } from 'vue-i18n'
import { 
  MessageSquare, 
  Box, 
  ChartBar, 
  Globe, 
  ArrowRight,
  ArrowUpRight 
} from 'lucide-vue-next'
import gsap from 'gsap'
import BackgroundEffect from '../components/common/BackgroundEffect.vue'

const router = useRouter()
const { t } = useI18n()

const features = [
  {
    title: t('home.featureChatTitle'),
    description: t('home.featureChatDesc'),
    icon: MessageSquare,
    path: '/chat'
  },
  {
    title: t('home.featureTranslatorTitle'),
    description: t('home.featureTranslatorDesc'),
    icon: Globe,
    path: '/translator'
  },
  {
    title: t('home.featureModelsTitle'),
    description: t('home.featureModelsDesc'),
    icon: Box,
    path: '/models'
  },
  {
    title: t('home.featureStatsTitle'),
    description: t('home.featureStatsDesc'),
    icon: ChartBar,
    path: '/dashboard'
  }
]

const titleRef = ref(null)
const titles = ['AI Toolbox', 'Creative Toolbox', 'Reliable Toolbox', 'Secure Toolbox', 'Local Toolbox']

// Animation definitions
const animations = [
  // 1. 3D Flip (Original)
  {
    out: { rotationX: -90, y: -50, opacity: 0, filter: 'blur(10px)', duration: 0.6, ease: "power2.in" },
    set: { rotationX: 90, y: 50, filter: 'blur(10px)' },
    in: { rotationX: 0, y: 0, opacity: 1, filter: 'blur(0px)', duration: 0.8, ease: "back.out(1.2)" }
  },
  // 2. Slide Left/Right
  {
    out: { x: -100, opacity: 0, skewX: 20, duration: 0.5, ease: "power2.in" },
    set: { x: 100, skewX: -20 },
    in: { x: 0, opacity: 1, skewX: 0, duration: 0.7, ease: "power4.out" }
  },
  // 3. Zoom Out/In (Cinematic)
  {
    out: { scale: 1.5, opacity: 0, filter: 'blur(15px)', duration: 0.6, ease: "power2.in" },
    set: { scale: 0.5, filter: 'blur(15px)' },
    in: { scale: 1, opacity: 1, filter: 'blur(0px)', duration: 0.8, ease: "elastic.out(1, 0.75)" }
  },
  // 4. Reveal Up (Elegant)
  {
    out: { y: -80, opacity: 0, scale: 0.9, duration: 0.5, ease: "power3.in" },
    set: { y: 80, scale: 0.9 },
    in: { y: 0, opacity: 1, scale: 1, duration: 0.7, ease: "power3.out" }
  },
  // 5. Glitch Snap (Tech)
  {
    out: { opacity: 0, scaleX: 1.2, filter: 'brightness(2)', duration: 0.1, ease: "rough" },
    set: { scaleX: 0.8, filter: 'brightness(2)' },
    in: { opacity: 1, scaleX: 1, filter: 'brightness(1)', duration: 0.2, ease: "rough" }
  }
]

onMounted(async () => {
  await nextTick()

  // 1. Full Title Rotation Animation with Varied Effects
  const element = titleRef.value
  const tlLoop = gsap.timeline({ repeat: -1 })
  
  // Initial state setup
  gsap.set(element, { 
    opacity: 1, 
    rotationX: 0,
    y: 0,
    x: 0,
    scale: 1,
    skewX: 0,
    filter: 'blur(0px) brightness(1)'
  })

  titles.forEach((title, index) => {
    // Pick an animation style (cycle through them)
    // We use (index % animations.length) to pick different styles
    // But since the first one (AI Toolbox) is already shown, we start transitions from the first switch
    
    const animIndex = index % animations.length
    const anim = animations[animIndex]

    // Animate OUT current title
    tlLoop.to(element, {
      ...anim.out,
      delay: 2.0 // Display time
    })
    
    // Set New Text & Prepare IN state
    .add(() => {
      const nextIndex = (index + 1) % titles.length
      element.textContent = titles[nextIndex]
      gsap.set(element, { 
        rotationX: 0, y: 0, x: 0, scale: 1, skewX: 0, filter: 'none', // Reset all potential props first
        ...anim.set 
      })
    })

    // Animate IN new title
    .to(element, {
      ...anim.in
    })
  })

  // 2. Initial Page Load Animation
  const tlInit = gsap.timeline({ defaults: { ease: 'power4.out' } })

  tlInit.to('.hero-badge', {
    y: 0,
    opacity: 1,
    duration: 0.8
  })
  .to(element, { // Fade in title initially
    opacity: 1,
    duration: 1,
    ease: "power2.out"
  }, '-=0.6')
  .to('.hero-subtitle', {
    y: 0,
    opacity: 1,
    duration: 0.8
  }, '-=0.6')
  .to('.hero-actions', {
    y: 0,
    opacity: 1,
    duration: 0.8
  }, '-=0.6')
  .to('.feature-card', {
    y: 0,
    opacity: 1,
    stagger: 0.1,
    duration: 0.8,
    ease: 'power2.out'
  }, '-=0.4')
})
</script>

<style scoped>
/* Custom animations that Tailwind doesn't support out of the box */
.animation-delay-2000 {
  animation-delay: 2s;
}

.perspective-1000 {
  perspective: 1000px;
}

.transform-style-3d {
  transform-style: preserve-3d;
}

/* Enhancing text rendering for large titles */
:root.dark .hero-title {
  text-shadow: 0 0 30px rgba(255,255,255,0.1);
}
</style>