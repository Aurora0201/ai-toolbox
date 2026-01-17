<template>
  <div class="home-container">
    <header class="home-header">
      <div class="hero-section">
        <div class="logo-animation">
          <span class="emoji-switcher">{{ currentEmoji }}</span>
        </div>
        <h1 class="slogan">
          <span
            v-for="(char, index) in sloganChars"
            :key="index"
            class="bouncing-char"
            :style="{ 
              animationDelay: `${index * 0.1}s`,
              color: index < 7 ? 'var(--primary-color)' : 'inherit'
            }"
          >{{ char === ' ' ? '&nbsp;' : char }}</span>
        </h1>
        <p class="app-description">
          {{ t.description }}
        </p>
        <div class="header-actions">
          <button
            class="btn btn-primary btn-lg"
            @click="router.push('/chat')"
          >
            {{ t.startChat }}
          </button>
          <button
            class="btn btn-outline btn-lg"
            @click="router.push('/models')"
          >
            {{ t.manageModels }}
          </button>
        </div>
      </div>
    </header>

    <div class="section-divider">
      <span class="divider-text">{{ t.exploreFeatures }}</span>
    </div>

    <main class="feature-grid">
      <FeatureCard
        :title="t.featureChatTitle"
        :description="t.featureChatDesc"
        @click="router.push('/chat')"
      >
        <template #icon>
          💬
        </template>
      </FeatureCard>

      <FeatureCard
        :title="t.featureModelsTitle"
        :description="t.featureModelsDesc"
        @click="router.push('/models')"
      >
        <template #icon>
          📦
        </template>
      </FeatureCard>

      <FeatureCard
        :title="t.featureStatsTitle"
        :description="t.featureStatsDesc"
        @click="router.push('/dashboard')"
      >
        <template #icon>
          📊
        </template>
      </FeatureCard>

      <FeatureCard
        :title="t.featureSettingsTitle"
        :description="t.featureSettingsDesc"
        @click="router.push('/settings')"
      >
        <template #icon>
          ⚙️
        </template>
      </FeatureCard>
    </main>

    <footer class="home-footer">
      <p class="text-muted">
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

<style scoped>
.home-container {
  padding: 64px 32px;
  max-width: 1100px;
  margin: 0 auto;
  display: flex;
  flex-direction: column;
  gap: 64px;
}

.home-header {
  text-align: center;
}

.hero-section {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 24px;
}

.logo-animation {
  font-size: 72px;
  height: 90px;
  display: flex;
  align-items: center;
  justify-content: center;
  filter: drop-shadow(0 0 20px rgba(13, 110, 253, 0.2));
}

.emoji-switcher {
  display: inline-block;
  animation: pulse 2s infinite ease-in-out;
}

@keyframes pulse {
  0%, 100% { transform: scale(1); }
  50% { transform: scale(1.1); }
}

.slogan {
  font-size: 48px;
  font-weight: 900;
  margin: 0;
  display: flex;
  justify-content: center;
  letter-spacing: -1px;
}

.bouncing-char {
  display: inline-block;
  animation: bounce 2s infinite ease-in-out;
}

@keyframes bounce {
  0%, 20%, 50%, 80%, 100% { transform: translateY(0); }
  40% { transform: translateY(-15px); }
  60% { transform: translateY(-7px); }
}

.app-description {
  font-size: 20px;
  color: var(--text-secondary);
  max-width: 700px;
  line-height: 1.6;
  margin: 0;
}

.header-actions {
  display: flex;
  gap: 16px;
  margin-top: 8px;
}

.btn-lg {
  padding: 12px 32px;
  font-size: 16px;
  border-radius: var(--radius-md);
}

.section-divider {
  display: flex;
  align-items: center;
  text-align: center;
  color: var(--text-muted);
}

.section-divider::before,
.section-divider::after {
  content: '';
  flex: 1;
  border-bottom: 1px solid var(--border-color);
}

.divider-text {
  padding: 0 20px;
  font-size: 12px;
  font-weight: 700;
  letter-spacing: 2px;
}

.feature-grid {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 32px;
}

.home-footer {
  margin-top: 32px;
  text-align: center;
  border-top: 1px solid var(--border-color);
  padding-top: 32px;
}

@media (max-width: 850px) {
  .feature-grid {
    grid-template-columns: 1fr;
  }
  .slogan {
    font-size: 36px;
  }
}
</style>
