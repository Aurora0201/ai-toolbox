<template>
  <div class="relative w-full max-w-3xl mx-auto transition-all duration-300">
    <!-- Floating Capsule Input -->
    <div 
      class="bg-background-surface border border-border rounded-[28px] shadow-lg transition-all duration-300 focus-within:border-primary/50 focus-within:ring-4 focus-within:ring-primary/10"
      :class="{ 'opacity-50 pointer-events-none': disabled && !isGenerating }"
    >
      <!-- Top Bar: Model & Language Selector -->
      <div class="flex items-center justify-between px-5 pt-3 pb-1">
        <div class="flex items-center gap-2">
          <!-- Model Selector -->
          <div class="relative group">
            <button
              class="flex items-center gap-1.5 px-2.5 py-1 bg-background-element rounded-full border border-border hover:border-primary/30 transition-all active:scale-95"
              @click="toggleModelDropdown"
            >
              <Cpu class="w-3.5 h-3.5 text-primary" />
              <span class="text-xs font-bold text-text-main">
                {{ selectedModel || $t('chat.selectModel') }}
              </span>
              <ChevronDown 
                class="w-3.5 h-3.5 text-text-sub transition-transform duration-200"
                :class="{ 'rotate-180': isModelDropdownOpen }"
              />
            </button>

            <!-- Model Dropdown Menu -->
            <transition
              enter-active-class="transition ease-out duration-200"
              enter-from-class="opacity-0 translate-y-1 scale-95"
              enter-to-class="opacity-100 translate-y-0 scale-100"
              leave-active-class="transition ease-in duration-150"
              leave-from-class="opacity-100 translate-y-0 scale-100"
              leave-to-class="opacity-0 translate-y-1 scale-95"
            >
              <div
                v-if="isModelDropdownOpen"
                class="absolute bottom-full left-0 mb-2 w-48 bg-background-surface border border-border rounded-lg shadow-xl py-1 z-50 overflow-hidden"
              >
                <div 
                  v-if="runningModels.length === 0"
                  class="px-3 py-2 text-xs text-text-sub italic"
                >
                  {{ $t('chat.noRunningModels') }}
                </div>
                <template v-else>
                  <button
                    v-for="model in runningModels"
                    :key="model.name"
                    class="w-full text-left px-3 py-2 text-xs font-medium transition-colors hover:bg-background-element flex items-center justify-between"
                    :class="selectedModel === model.name ? 'text-primary' : 'text-text-main'"
                    @click="selectModel(model.name)"
                  >
                    <span class="truncate">{{ model.name }}</span>
                    <Check 
                      v-if="selectedModel === model.name"
                      class="w-3 h-3" 
                    />
                  </button>
                </template>
              </div>
            </transition>
          </div>

          <!-- Target Language Selector -->
          <div class="flex items-center gap-2 text-xs text-text-sub">
            <ArrowRight class="w-3.5 h-3.5" />
          </div>

          <div class="relative group">
            <button
              class="flex items-center gap-1.5 px-2.5 py-1 bg-background-element rounded-full border border-border hover:border-primary/30 transition-all active:scale-95"
              @click="toggleLangDropdown"
            >
              <Languages class="w-3.5 h-3.5 text-secondary" />
              <span class="text-xs font-bold text-text-main">
                {{ targetLanguage }}
              </span>
              <ChevronDown 
                class="w-3.5 h-3.5 text-text-sub transition-transform duration-200"
                :class="{ 'rotate-180': isLangDropdownOpen }"
              />
            </button>

            <!-- Language Dropdown Menu -->
            <transition
              enter-active-class="transition ease-out duration-200"
              enter-from-class="opacity-0 translate-y-1 scale-95"
              enter-to-class="opacity-100 translate-y-0 scale-100"
              leave-active-class="transition ease-in duration-150"
              leave-from-class="opacity-100 translate-y-0 scale-100"
              leave-to-class="opacity-0 translate-y-1 scale-95"
            >
              <div
                v-if="isLangDropdownOpen"
                class="absolute bottom-full left-0 mb-2 w-40 bg-background-surface border border-border rounded-lg shadow-xl py-1 z-50 overflow-hidden max-h-60 overflow-y-auto"
              >
                <button
                  v-for="lang in availableLanguages"
                  :key="lang"
                  class="w-full text-left px-3 py-2 text-xs font-medium transition-colors hover:bg-background-element flex items-center justify-between"
                  :class="targetLanguage === lang ? 'text-primary' : 'text-text-main'"
                  @click="selectLanguage(lang)"
                >
                  <span>{{ lang }}</span>
                  <Check 
                    v-if="targetLanguage === lang"
                    class="w-3 h-3" 
                  />
                </button>
              </div>
            </transition>
          </div>
          
          <button 
            v-if="messagesCount > 0"
            class="flex items-center gap-1.5 px-2.5 py-1 hover:bg-danger/10 text-danger rounded-full border border-transparent hover:border-danger/20 transition-all text-xs font-bold uppercase"
            @click="handleClear"
          >
            <Trash2 class="w-3.5 h-3.5" />
            {{ $t('chat.clear') }}
          </button>
        </div>
      </div>

      <!-- Textarea Area -->
      <div class="px-5 py-2">
        <textarea
          ref="inputRef"
          v-model="text"
          :placeholder="placeholder"
          class="w-full bg-transparent border-none focus:ring-0 text-text-main text-[15px] leading-relaxed resize-none max-h-[200px] overflow-y-auto py-1 scrollbar-hide outline-none appearance-none"
          rows="1"
          @input="adjustHeight"
          @keydown="handleKeydown"
        />
      </div>

      <!-- Bottom Bar: Status & Send -->
      <div class="flex items-center justify-between px-5 pb-3 pt-1">
        <div class="flex flex-wrap gap-2 items-center">
          <span
            v-if="text.length > 0"
            class="text-[10px] font-bold text-text-sub uppercase tracking-wider font-mono"
          >
            {{ text.length }} chars
          </span>
        </div>

        <button
          class="flex items-center justify-center w-10 h-10 rounded-full transition-all"
          :class="canSend || isGenerating ? 'bg-primary text-white shadow-lg shadow-primary/20 hover:scale-105 active:scale-95' : 'bg-background-element text-text-sub'"
          :disabled="!canSend && !isGenerating"
          @click="handleSend"
        >
          <template v-if="isGenerating">
            <Square class="w-4 h-4 fill-current animate-pulse" />
          </template>
          <template v-else>
            <ArrowUp class="w-5 h-5 stroke-[2.5]" />
          </template>
        </button>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, onMounted, onUnmounted, nextTick } from 'vue'
import { Cpu, ImagePlus, Trash2, ArrowUp, Square, X, ChevronDown, Check } from 'lucide-vue-next'
import { useConfirm } from '../../../composables/useConfirm'
import { useI18n } from 'vue-i18n'

const props = defineProps({
  selectedModel: {
    type: String,
    default: ''
  },
  runningModels: {
    type: Array,
    default: () => []
  },
  targetLanguage: {
    type: String,
    default: 'English'
  },
  isGenerating: Boolean,
  messagesCount: {
    type: Number,
    default: 0
  },
  disabled: Boolean,
  placeholder: {
    type: String,
    default: 'Enter text to translate...'
  }
})

const emit = defineEmits(['send', 'stop', 'clear', 'update:selectedModel', 'update:targetLanguage'])

const { confirm } = useConfirm()
const { t } = useI18n()
const text = ref('')
const inputRef = ref(null)
const isModelDropdownOpen = ref(false)
const isLangDropdownOpen = ref(false)

const availableLanguages = [
  'English',
  'Chinese',
  'Spanish',
  'French',
  'German',
  'Japanese',
  'Korean',
  'Russian',
  'Italian',
  'Portuguese'
]

const canSend = computed(() => {
  return text.value.trim().length > 0
})

const toggleModelDropdown = (e) => {
  e.stopPropagation()
  isModelDropdownOpen.value = !isModelDropdownOpen.value
  isLangDropdownOpen.value = false
}

const toggleLangDropdown = (e) => {
  e.stopPropagation()
  isLangDropdownOpen.value = !isLangDropdownOpen.value
  isModelDropdownOpen.value = false
}

const selectModel = (name) => {
  emit('update:selectedModel', name)
  isModelDropdownOpen.value = false
}

const selectLanguage = (lang) => {
  emit('update:targetLanguage', lang)
  isLangDropdownOpen.value = false
}

const closeDropdowns = () => {
  isModelDropdownOpen.value = false
  isLangDropdownOpen.value = false
}

const adjustHeight = () => {
  const el = inputRef.value
  if (!el) return
  el.style.height = 'auto'
  el.style.height = `${el.scrollHeight}px`
}

const handleKeydown = (e) => {
  if (e.key === 'Enter' && !e.shiftKey) {
    e.preventDefault()
    handleSend()
  }
}

const handleSend = () => {
  if (props.isGenerating) {
    emit('stop')
    return
  }
  
  if (!canSend.value || props.disabled) return
  
  emit('send', {
    text: text.value
  })
  
  text.value = ''
  nextTick(() => adjustHeight())
}

const handleClear = async () => {
  const ok = await confirm({
    title: t('chat.confirmClearTitle'),
    message: t('chat.confirmClearMsg'),
    confirmText: t('chat.confirmClearBtn'),
    confirmType: 'danger'
  })
  if (ok) emit('clear')
}

onMounted(() => {
  inputRef.value?.focus()
  window.addEventListener('click', closeDropdowns)
})

onUnmounted(() => {
  window.removeEventListener('click', closeDropdowns)
})

defineExpose({
  focus: () => inputRef.value?.focus()
})
</script>

<style scoped>
.scrollbar-hide::-webkit-scrollbar {
  display: none;
}
.scrollbar-hide {
  -ms-overflow-style: none;
  scrollbar-width: none;
}
</style>