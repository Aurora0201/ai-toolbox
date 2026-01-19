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
          {{ t.description }}
        </p>
        <div class="flex gap-4 mt-2">
          <button
            class="px-8 py-3 text-base rounded-md bg-primary text-white hover:bg-primary-hover transition-colors font-medium shadow-sm"
            @click="router.push('/chat')"
          >
            {{ t.startChat }}
          </button>
          <button
            class="px-8 py-3 text-base rounded-md border border-border text-text-main hover:bg-background-element hover:text-primary hover:border-primary transition-all font-medium bg-transparent"
            @click="router.push('/models')"
          >
            {{ t.manageModels }}
          </button>
        </div>
      </div>
    </header>

    <div class="flex items-center text-center text-text-sub before:flex-1 before:border-b before:border-border after:flex-1 after:border-b after:border-border">
      <span class="px-5 text-xs font-bold tracking-widest uppercase">{{ t.exploreFeatures }}</span>
    </div>

    <main class="grid grid-cols-1 md:grid-cols-2 gap-8">
      <FeatureCard
        :title="t.featureChatTitle"
        :description="t.featureChatDesc"
        @click="router.push('/chat')"
      >
        <template #icon>
          <MessageSquare class="w-8 h-8 text-primary" />
        </template>
      </FeatureCard>

      <FeatureCard
        :title="t.featureModelsTitle"
        :description="t.featureModelsDesc"
        @click="router.push('/models')"
      >
        <template #icon>
          <Box class="w-8 h-8 text-primary" />
        </template>
      </FeatureCard>

      <FeatureCard
        :title="t.featureStatsTitle"
        :description="t.featureStatsDesc"
        @click="router.push('/dashboard')"
      >
        <template #icon>
          <ChartBar class="w-8 h-8 text-primary" />
        </template>
      </FeatureCard>

      <FeatureCard
        :title="t.featureSettingsTitle"
        :description="t.featureSettingsDesc"
        @click="router.push('/settings')"
      >
        <template #icon>
          <Settings class="w-8 h-8 text-primary" />
        </template>
      </FeatureCard>
    </main>

    <footer class="mt-8 text-center border-t border-border pt-8 text-text-sub">
      <p class="text-sm">
        {{ t.footer }}
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
import { useSettingsStore } from '../store/settings'
import FeatureCard from '../components/common/FeatureCard.vue'
import { MessageSquare, Box, ChartBar, Settings } from 'lucide-vue-next'

const router = useRouter()
const settings = useSettingsStore()

const translations = {
  en: {
    slogan: 'Empower Your Local AI',
    description: 'Welcome to AI Toolbox, your high-performance desktop gateway to local large language models. Manage your Ollama library, monitor real-time generation, and analyze your usage with a professional, developer-centric interface.',
    startChat: 'Start Chatting',
    manageModels: 'Manage Models',
    exploreFeatures: 'EXPLORE FEATURES',
    featureChatTitle: 'Intelligent Chat',
    featureChatDesc: 'Connect with any local model instantly. Experience low-latency, private, and secure AI interactions.',
    featureModelsTitle: 'Model Management',
    featureModelsDesc: 'Seamlessly pull, update, and manage your Ollama model library with advanced monitoring.',
    featureStatsTitle: 'Usage Analytics',
    featureStatsDesc: 'Deep dive into your local AI consumption. Track tokens, performance, and historical trends.',
    featureSettingsTitle: 'Advanced Settings',
    featureSettingsDesc: 'Customize your model parameters, server connections, and application preferences.',
    footer: 'Built for the local AI community. Powered by Tauri & Ollama.'
  },
  zh: {
    slogan: '赋能您的本地 AI',
    description: '欢迎使用 AI Toolbox，这是您通往本地大语言模型的高性能桌面门户。通过专业的、以开发者为中心的界面，管理您的 Ollama 库，监控实时生成，并分析您的使用情况。',
    startChat: '开始对话',
    manageModels: '管理模型',
    exploreFeatures: '探索功能',
    featureChatTitle: '智能对话',
    featureChatDesc: '立即连接任何本地模型。体验低延迟、私密且安全的 AI 交互。',
    featureModelsTitle: '模型管理',
    featureModelsDesc: '无缝拉取、更新和管理您的 Ollama 模型库，并配有高级监控。',
    featureStatsTitle: '用量分析',
    featureStatsDesc: '深入了解您的本地 AI 消耗。追踪 Token、性能和历史趋势。',
    featureSettingsTitle: '高级设置',
    featureSettingsDesc: '自定义您的模型参数、服务器连接和应用程序首选项。',
    footer: '为本地 AI 社区打造。由 Tauri & Ollama 驱动。'
  }
}

const t = computed(() => translations[settings.language] || translations.en)

const sloganChars = computed(() => t.value.slogan.split(''))

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

