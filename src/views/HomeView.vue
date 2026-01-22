<template>
  <div class="py-16 px-8 max-w-[1100px] mx-auto flex flex-col gap-16">
    <header class="text-center">
      <div class="flex flex-col items-center gap-6">
        <div class="text-7xl h-[90px] flex items-center justify-center drop-shadow-[0_0_20px_rgba(37,99,235,0.2)]">
          <span class="inline-block animate-pulse">{{ currentEmoji }}</span>
        </div>
        <h1 class="text-5xl font-black m-0 flex justify-center tracking-tight text-text-main">
          <span
            v-for="(char, index) in sloganChars"
            :key="index"
            class="inline-block animate-bounce"
            :style="{ 
              animationDelay: `${index * 0.1}s`,
              color: index < 7 ? 'var(--primary)' : 'inherit'
            }"
          >{{ char === ' ' ? '&nbsp;' : char }}</span>
        </h1>
        <p class="text-xl text-text-sub max-w-[700px] leading-relaxed m-0">
          {{ $t('home.description') }}
        </p>
        <div class="flex gap-4 mt-2">
          <button
            class="px-8 py-3 text-base rounded-md bg-primary text-white hover:bg-primary-hover transition-colors font-medium shadow-sm"
            @click="router.push('/chat')"
          >
            {{ $t('home.startChat') }}
          </button>
          <button
            class="px-8 py-3 text-base rounded-md border border-border text-text-main hover:bg-background-element hover:text-primary hover:border-primary transition-all font-medium bg-transparent"
            @click="router.push('/models')"
          >
            {{ $t('home.manageModels') }}
          </button>
        </div>
      </div>
    </header>

    <div class="flex items-center text-center text-text-sub before:flex-1 before:border-b before:border-border after:flex-1 after:border-b after:border-border">
      <span class="px-5 text-xs font-bold tracking-widest uppercase">{{ $t('home.exploreFeatures') }}</span>
    </div>

    <main class="grid grid-cols-1 md:grid-cols-2 gap-8">
      <FeatureCard
        :title="$t('home.featureChatTitle')"
        :description="$t('home.featureChatDesc')"
        @click="router.push('/chat')"
      >
        <template #icon>
          <MessageSquare class="w-8 h-8 text-primary" />
        </template>
      </FeatureCard>

      <FeatureCard
        :title="$t('home.featureTranslatorTitle')"
        :description="$t('home.featureTranslatorDesc')"
        @click="router.push('/translator')"
      >
        <template #icon>
          <Globe class="w-8 h-8 text-primary" />
        </template>
      </FeatureCard>

      <FeatureCard
        :title="$t('home.featureModelsTitle')"
        :description="$t('home.featureModelsDesc')"
        @click="router.push('/models')"
      >
        <template #icon>
          <Box class="w-8 h-8 text-primary" />
        </template>
      </FeatureCard>

      <FeatureCard
        :title="$t('home.featureStatsTitle')"
        :description="$t('home.featureStatsDesc')"
        @click="router.push('/dashboard')"
      >
        <template #icon>
          <ChartBar class="w-8 h-8 text-primary" />
        </template>
      </FeatureCard>

      <FeatureCard
        :title="$t('home.featureSettingsTitle')"
        :description="$t('home.featureSettingsDesc')"
        @click="router.push('/settings')"
      >
        <template #icon>
          <Settings class="w-8 h-8 text-primary" />
        </template>
      </FeatureCard>
    </main>

    <footer class="mt-8 text-center border-t border-border pt-8 text-text-sub">
      <p class="text-sm">
        {{ $t('home.footer') }}
      </p>
    </footer>
  </div>
</template>

<script setup>
/**
 * Refined Home view with enhanced animations, professional descriptions, 
 * and improved visual hierarchy.
 */
import { ref, onMounted, onUnmounted, computed } from 'vue'
import { useRouter } from 'vue-router'
import FeatureCard from '../components/common/FeatureCard.vue'
import { MessageSquare, Box, ChartBar, Settings, Globe } from 'lucide-vue-next'
import { useI18n } from 'vue-i18n'

const router = useRouter()
const { t } = useI18n()

const sloganChars = computed(() => t('home.slogan').split(''))

const emojis = ['🤖', '🧠', '🚀', '🛠️', '✨', '📡', '💻', '🔮']
const currentEmoji = ref(emojis[0])
let emojiInterval

onMounted(() => {
  // Rotate emojis every 2 seconds with a subtle transition effect
  emojiInterval = setInterval(() => {
    const currentIndex = emojis.indexOf(currentEmoji.value)
    currentEmoji.value = emojis[(currentIndex + 1) % emojis.length]
  }, 2000)
})

onUnmounted(() => {
  if (emojiInterval) clearInterval(emojiInterval)
})
</script>
