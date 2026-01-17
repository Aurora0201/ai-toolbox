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
          Welcome to <strong>AI Toolbox</strong>, your high-performance desktop gateway to local large language models. 
          Manage your Ollama library, monitor real-time generation, and analyze your usage with a professional, developer-centric interface.
        </p>
        <div class="header-actions">
          <button
            class="btn btn-primary btn-lg"
            @click="router.push('/chat')"
          >
            Start Chatting
          </button>
          <button
            class="btn btn-outline btn-lg"
            @click="router.push('/models')"
          >
            Manage Models
          </button>
        </div>
      </div>
    </header>

    <div class="section-divider">
      <span class="divider-text">EXPLORE FEATURES</span>
    </div>

    <main class="feature-grid">
      <FeatureCard
        title="Intelligent Chat"
        description="Connect with any local model instantly. Experience low-latency, private, and secure AI interactions."
        @click="router.push('/chat')"
      >
        <template #icon>
          💬
        </template>
      </FeatureCard>

      <FeatureCard
        title="Model Management"
        description="Seamlessly pull, update, and manage your Ollama model library with advanced monitoring."
        @click="router.push('/models')"
      >
        <template #icon>
          📦
        </template>
      </FeatureCard>

      <FeatureCard
        title="Usage Analytics"
        description="Deep dive into your local AI consumption. Track tokens, performance, and historical trends."
        @click="router.push('/dashboard')"
      >
        <template #icon>
          📊
        </template>
      </FeatureCard>

      <FeatureCard
        title="Advanced Settings"
        description="Customize your model parameters, server connections, and application preferences."
        @click="router.push('/settings')"
      >
        <template #icon>
          ⚙️
        </template>
      </FeatureCard>
    </main>

    <footer class="home-footer">
      <p class="text-muted">
        Built for the local AI community. Powered by Tauri & Ollama.
      </p>
    </footer>
  </div>
</template>

<script setup>
/**
 * Refined Home view with enhanced animations, professional descriptions, 
 * and improved visual hierarchy.
 */
import { ref, onMounted, onUnmounted } from 'vue'
import { useRouter } from 'vue-router'
import FeatureCard from '../components/common/FeatureCard.vue'

const router = useRouter()
const slogan = 'Empower Your Local AI'
const sloganChars = slogan.split('')

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
