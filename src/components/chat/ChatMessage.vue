<template>
  <div 
    class="flex gap-4 p-4 transition-colors"
    :class="role === 'user' ? 'bg-transparent' : 'bg-background-element dark:bg-transparent'"
  >
    <!-- Avatar -->
    <div class="shrink-0 pt-1">
      <div 
        class="w-9 h-9 rounded-lg flex items-center justify-center shadow-sm"
        :class="role === 'user' ? 'bg-primary text-white' : 'bg-background-surface border border-border text-primary'"
      >
        <User
          v-if="role === 'user'"
          class="w-5 h-5"
        />
        <Bot
          v-else
          class="w-5 h-5"
        />
      </div>
    </div>

    <!-- Content -->
    <div class="flex-1 min-w-0">
      <div class="flex items-center gap-2 mb-1.5">
        <span class="text-[11px] font-black uppercase tracking-widest text-text-sub font-mono">
          {{ role === 'user' ? $t('chat.you') : $t('chat.ai') }}
        </span>
        <span class="text-[10px] text-text-sub opacity-50 font-mono">
          {{ formatTime(timestamp) }}
        </span>
      </div>

      <!-- Thinking Block (if any) -->
      <div
        v-if="thinking"
        class="mb-4"
      >
        <details
          class="group border border-border rounded-lg bg-background-element overflow-hidden"
          :open="isGenerating"
        >
          <summary class="flex items-center gap-2 p-2.5 cursor-pointer hover:bg-background-element transition-colors select-none list-none">
            <div class="w-5 h-5 flex items-center justify-center rounded-md bg-background-surface border border-border group-open:rotate-180 transition-transform">
              <ChevronDown class="w-3 h-3" />
            </div>
            <span class="text-xs font-bold uppercase tracking-wider text-text-sub flex items-center gap-2">
              <Brain class="w-3.5 h-3.5 text-primary" />
              {{ isThinking ? $t('chat.thinking') : $t('chat.thought_completed') }}
            </span>
            <div
              v-if="isThinking"
              class="flex gap-1"
            >
              <span
                class="w-1 h-1 bg-primary rounded-full animate-bounce"
                style="animation-delay: 0s"
              />
              <span
                class="w-1 h-1 bg-primary rounded-full animate-bounce"
                style="animation-delay: 0.1s"
              />
              <span
                class="w-1 h-1 bg-primary rounded-full animate-bounce"
                style="animation-delay: 0.2s"
              />
            </div>
          </summary>
          <div class="p-3 pt-1 text-sm text-text-sub leading-relaxed font-sans italic whitespace-pre-wrap border-t border-border">
            {{ thinking }}
          </div>
        </details>
      </div>

      <!-- Message Content -->
      <!-- eslint-disable-next-line vue/no-v-html -->
      <div 
        class="markdown-body text-text-main relative inline-block w-full"
        v-html="renderedContent"
      />
      
      <!-- Blinking Cursor -->
      <span 
        v-if="isGenerating && !isThinking" 
        class="inline-block w-2 h-4 bg-primary rounded-full ml-1 align-middle cursor-blink"
      />
    </div>
  </div>
</template>

<script setup>
import { computed } from 'vue'
import { User, Bot, ChevronDown, Brain } from 'lucide-vue-next'
import MarkdownIt from 'markdown-it'
import hljs from 'highlight.js'
import DOMPurify from 'dompurify'

const props = defineProps({
  role: {
    type: String,
    default: 'user'
  },
  content: {
    type: String,
    default: ''
  },
  thinking: {
    type: String,
    default: ''
  },
  isThinking: {
    type: Boolean,
    default: false
  },
  timestamp: {
    type: Number,
    default: () => Date.now()
  },
  isGenerating: Boolean
})

const md = new MarkdownIt({
  html: false,
  linkify: true,
  typographer: true,
  highlight: function (str, lang) {
    if (lang && hljs.getLanguage(lang)) {
      try {
        return '<pre class="hljs"><code>' +
               hljs.highlight(str, { language: lang, ignoreIllegals: true }).value +
               '</code></pre>';
      } catch (__) {}
    }
    return '<pre class="hljs"><code>' + md.utils.escapeHtml(str) + '</code></pre>';
  }
})

const renderedContent = computed(() => {
  if (!props.content) return ''
  const rawHtml = md.render(props.content)
  return DOMPurify.sanitize(rawHtml)
})

const formatTime = (ts) => {
  if (!ts) return ''
  return new Date(ts).toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' })
}
</script>
