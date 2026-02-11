<template>
  <div class="relative w-full h-full bg-[#2d2d2d] border border-[#404040] border-t-0 rounded-b-lg overflow-hidden flex flex-row">
    <!-- Left Column: Line Numbers -->
    <div class="relative flex-shrink-0 w-[48px] h-full bg-[#2d2d2d] border-r border-[#404040] z-20 overflow-hidden text-right select-none">
      <!-- 
        Line Numbers Wrapper 
        - Absolute: Prevents stretching parent height.
        - Transform: Syncs with textarea scroll.
      -->
      <div 
        class="absolute top-0 left-0 w-full py-4 pr-3 text-[#6e7681] font-mono text-[14px] leading-[21px]"
        :style="{ transform: `translateY(-${scrollTop}px)` }"
      >
        <div
          v-for="n in lineCount"
          :key="n"
        >
          {{ n }}
        </div>
      </div>
    </div>

    <!-- Right Column: Editor Area -->
    <div class="relative flex-1 min-h-0 min-w-0">
      <!-- Top Layer: Input -->
      <textarea
        ref="textareaRef"
        class="editor-common code-textarea block w-full h-full bg-transparent text-transparent caret-white resize-none outline-none z-10 relative"
        :value="modelValue"
        spellcheck="false"
        wrap="off"
        @input="handleInput"
        @scroll="syncScroll"
      />

      <!-- Bottom Layer: Syntax Highlighting -->
      <pre
        ref="preRef"
        class="editor-common absolute inset-0 w-full h-full pointer-events-none z-0"
        aria-hidden="true"
      ><!-- eslint-disable-next-line vue/no-v-html --><code
          :class="`language-${language}`"
          v-html="highlightedCode"
      /></pre>
    </div>
  </div>
</template>

<script setup>
import { ref, computed } from 'vue'
import Prism from 'prismjs'
import 'prismjs/themes/prism-tomorrow.css'
import 'prismjs/components/prism-markup'
import 'prismjs/components/prism-css'
import 'prismjs/components/prism-javascript'

const props = defineProps({
  modelValue: {
    type: String,
    default: ''
  },
  language: {
    type: String,
    default: 'html'
  }
})

const emit = defineEmits(['update:modelValue'])

const textareaRef = ref(null)
const preRef = ref(null)
const scrollTop = ref(0) // Added back for sync

const lineCount = computed(() => {
  return (props.modelValue || '').split('\n').length
})

const highlightedCode = computed(() => {
  const code = props.modelValue || ''
  const safeCode = code.endsWith('\n') ? code + ' ' : code
  return Prism.highlight(safeCode, Prism.languages[props.language] || Prism.languages.html, props.language)
})

const handleInput = (e) => {
  emit('update:modelValue', e.target.value)
}

const syncScroll = () => {
  if (textareaRef.value) {
    const { scrollTop: top, scrollLeft: left } = textareaRef.value
    
    // Sync line numbers
    scrollTop.value = top
    
    // Sync highlight layer
    if (preRef.value) {
      preRef.value.scrollTop = top
      preRef.value.scrollLeft = left
    }
  }
}
</script>

<style scoped>
/* Typography & Alignment */
.editor-common {
  font-family: 'Roboto Mono', 'Menlo', 'Monaco', 'Courier New', monospace !important;
  font-size: 14px !important;
  line-height: 21px !important;
  
  white-space: pre !important;
  word-wrap: normal !important;
  overflow-wrap: normal !important;
  word-break: keep-all !important;
  
  box-sizing: border-box !important;
  overflow: auto !important;
  tab-size: 2 !important;
  
  /* Shared Layout */
  padding: 16px !important;
  margin: 0 !important;
  border: 0 !important;
}

/* Textarea Specifics */
.code-textarea {
  color: transparent !important;
  background: transparent !important;
  -webkit-text-fill-color: transparent;
  -webkit-appearance: none;
  appearance: none;
}

/* Selection Visibility */
.code-textarea::selection {
  background-color: rgba(38, 79, 120, 0.7) !important;
  color: transparent !important;
}

/* Scrollbar Styles */
.editor-common::-webkit-scrollbar {
  width: 12px;
  height: 12px;
}
.editor-common::-webkit-scrollbar-track {
  background: #2d2d2d;
}
.editor-common::-webkit-scrollbar-thumb {
  background: #424242;
  border: 2px solid #2d2d2d;
  background-clip: content-box;
  border-radius: 6px;
}
.editor-common::-webkit-scrollbar-thumb:hover {
  background: #555;
}
.editor-common::-webkit-scrollbar-corner {
  background: #2d2d2d;
}

/* Highlighting layer specifics */
pre {
  background: transparent !important;
}

/* Force Reset for Prism styles */
:deep(code), 
:deep(span) {
  font-weight: normal !important;
  font-style: normal !important;
  font-family: inherit !important;
}
</style>