<template>
  <div class="relative w-full max-w-3xl mx-auto transition-all duration-300">
    <!-- Floating Capsule Input -->
    <div 
      class="bg-background-surface border border-border rounded-[28px] shadow-lg transition-all duration-300 focus-within:border-primary/50 focus-within:ring-4 focus-within:ring-primary/10"
      :class="{ 'opacity-50 pointer-events-none': disabled && !isGenerating }"
    >
      <!-- Top Bar: Model Selector & Actions -->
      <div class="flex items-center justify-between px-5 pt-3 pb-1">
        <div class="flex items-center gap-2">
          <div class="relative group">
            <button
              class="flex items-center gap-1.5 px-2.5 py-1 bg-background-element rounded-full border border-border hover:border-primary/30 transition-all active:scale-95"
              @click="toggleDropdown"
            >
              <Cpu class="w-3.5 h-3.5 text-primary" />
              <span class="text-xs font-bold text-text-main">
                {{ selectedModel || $t('chat.selectModel') }}
              </span>
              <ChevronDown 
                class="w-3.5 h-3.5 text-text-sub transition-transform duration-200"
                :class="{ 'rotate-180': isDropdownOpen }"
              />
            </button>

            <!-- Custom Dropdown Menu -->
            <transition
              enter-active-class="transition ease-out duration-200"
              enter-from-class="opacity-0 translate-y-1 scale-95"
              enter-to-class="opacity-100 translate-y-0 scale-100"
              leave-active-class="transition ease-in duration-150"
              leave-from-class="opacity-100 translate-y-0 scale-100"
              leave-to-class="opacity-0 translate-y-1 scale-95"
            >
              <div
                v-if="isDropdownOpen"
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
          
          <button 
            v-if="messagesCount > 0"
            class="flex items-center gap-1.5 px-2.5 py-1 hover:bg-danger/10 text-danger rounded-full border border-transparent hover:border-danger/20 transition-all text-xs font-bold uppercase"
            @click="handleClear"
          >
            <Trash2 class="w-3.5 h-3.5" />
            {{ $t('chat.clear') }}
          </button>
        </div>

        <div class="flex items-center gap-2">
          <input
            ref="fileInput"
            type="file"
            accept="image/*"
            class="hidden"
            @change="handleFileChange"
          >
          <button 
            class="p-1.5 text-text-sub hover:text-primary transition-colors rounded-full hover:bg-primary/10"
            :title="$t('chat.uploadImage')"
            @click="$refs.fileInput.click()"
          >
            <ImagePlus class="w-4 h-4" />
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
          @paste="handlePaste"
        />
      </div>

      <!-- Bottom Bar: Status & Send -->
      <div class="flex items-center justify-between px-5 pb-3 pt-1">
        <div class="flex flex-wrap gap-2 items-center">
          <div 
            v-for="(img, idx) in images" 
            :key="idx"
            class="group relative w-12 h-12 rounded-lg border border-border overflow-hidden bg-background-element"
          >
            <img
              :src="img"
              class="w-full h-full object-cover"
            >
            <button 
              class="absolute inset-0 bg-black/40 opacity-0 group-hover:opacity-100 flex items-center justify-center transition-opacity"
              @click="removeImage(idx)"
            >
              <X class="w-3.5 h-3.5 text-white" />
            </button>
          </div>
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
import { useConfirm } from '../../composables/useConfirm'
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
  isGenerating: Boolean,
  messagesCount: {
    type: Number,
    default: 0
  },
  disabled: Boolean,
  placeholder: {
    type: String,
    default: 'Type a message...'
  }
})

const emit = defineEmits(['send', 'stop', 'clear', 'update:selectedModel'])

const { confirm } = useConfirm()
const { t } = useI18n()
const text = ref('')
const images = ref([])
const inputRef = ref(null)
const fileInput = ref(null)
const isDropdownOpen = ref(false)

const canSend = computed(() => {
  return text.value.trim().length > 0 || images.value.length > 0
})

const toggleDropdown = (e) => {
  e.stopPropagation()
  isDropdownOpen.value = !isDropdownOpen.value
}

const selectModel = (name) => {
  emit('update:selectedModel', name)
  isDropdownOpen.value = false
}

const closeDropdown = () => {
  isDropdownOpen.value = false
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
    text: text.value,
    images: [...images.value]
  })
  
  text.value = ''
  images.value = []
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

const handleFileChange = (e) => {
  const files = Array.from(e.target.files)
  files.forEach(processFile)
  fileInput.value.value = ''
}

const handlePaste = (e) => {
  const items = (e.clipboardData || e.originalEvent.clipboardData).items
  for (const item of items) {
    if (item.type.indexOf('image') !== -1) {
      const file = item.getAsFile()
      processFile(file)
    }
  }
}

const processFile = (file) => {
  if (!file) return
  const reader = new FileReader()
  reader.onload = (e) => {
    images.value.push(e.target.result)
  }
  reader.readAsDataURL(file)
}

const removeImage = (idx) => {
  images.value.splice(idx, 1)
}

onMounted(() => {
  inputRef.value?.focus()
  window.addEventListener('click', closeDropdown)
})

onUnmounted(() => {
  window.removeEventListener('click', closeDropdown)
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
