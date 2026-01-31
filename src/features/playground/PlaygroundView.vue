<template>
  <div class="p-8 h-full flex flex-col max-w-[1200px] mx-auto">
    <div class="mb-6 shrink-0">
      <h1 class="text-2xl font-bold text-text-main">
        {{ $t('playground.title') }}
      </h1>
      <p class="text-text-sub mt-1">
        {{ $t('playground.subtitle') }}
      </p>
    </div>

    <!-- Main Editor Area -->
    <div class="flex-1 min-h-0 flex flex-col gap-4">
      <div class="flex-1 min-h-0 bg-background-surface border border-border rounded-lg shadow-sm overflow-hidden flex flex-col">
        <!-- Toolbar -->
        <div class="px-4 py-3 border-b border-border bg-background-element flex justify-between items-center">
          <div class="flex items-center gap-2 text-sm font-semibold text-text-main">
            <Code2 class="w-4 h-4 text-primary" />
            <span>index.html</span>
          </div>
          <button
            class="px-4 py-1.5 rounded-md font-medium text-sm transition-colors bg-primary text-white hover:bg-primary-hover flex items-center gap-2 shadow-sm hover:shadow-md"
            @click="runCode"
          >
            <Play class="w-3.5 h-3.5 fill-current" />
            {{ $t('playground.run') }}
          </button>
        </div>
        
        <!-- Editor -->
        <div class="flex-1 relative">
          <CodeEditor
            v-model="htmlCode"
            language="html"
          />
        </div>
      </div>
    </div>

    <!-- Fullscreen Preview Overlay -->
    <Transition
      enter-active-class="transition duration-300 ease-out"
      enter-from-class="translate-x-full opacity-0"
      enter-to-class="translate-x-0 opacity-100"
      leave-active-class="transition duration-200 ease-in"
      leave-from-class="translate-x-0 opacity-100"
      leave-to-class="translate-x-full opacity-0"
    >
      <div
        v-if="showPreview"
        class="fixed inset-0 top-8 z-[100] bg-background-app flex flex-col"
      >
        <!-- Preview Header -->
        <div class="h-14 border-b border-border bg-background-surface flex items-center justify-between px-6 shadow-sm shrink-0">
          <div class="flex items-center gap-2 font-bold text-lg text-text-main">
            <Globe class="w-5 h-5 text-success" />
            {{ $t('playground.previewTitle') }}
          </div>
          <button
            class="p-2 hover:bg-background-element rounded-full transition-colors text-text-sub hover:text-text-main"
            :title="$t('playground.close')"
            @click="closePreview"
          >
            <X class="w-6 h-6" />
          </button>
        </div>

        <!-- Iframe Container -->
        <div class="flex-1 bg-white relative">
          <iframe
            ref="previewFrame"
            class="absolute inset-0 w-full h-full border-none"
            sandbox="allow-scripts allow-modals"
            title="Preview"
          />
        </div>
      </div>
    </Transition>
  </div>
</template>

<script setup>
import { ref, onMounted } from 'vue'
import CodeEditor from './components/CodeEditor.vue'
import { Play, X, Code2, Globe } from 'lucide-vue-next'

const htmlCode = ref('')
const showPreview = ref(false)
const previewFrame = ref(null)

const defaultCode = `<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <style>
    body {
      font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif;
      display: flex;
      justify-content: center;
      align-items: center;
      height: 100vh;
      margin: 0;
      background: linear-gradient(135deg, #f5f7fa 0%, #c3cfe2 100%);
      color: #333;
    }
    .card {
      background: white;
      padding: 2rem;
      border-radius: 12px;
      box-shadow: 0 10px 25px rgba(0,0,0,0.1);
      text-align: center;
      max-width: 400px;
    }
    h1 { color: #2563EB; margin-bottom: 0.5rem; }
    p { color: #666; line-height: 1.6; }
    button {
      background: #2563EB;
      color: white;
      border: none;
      padding: 10px 20px;
      border-radius: 6px;
      font-size: 1rem;
      cursor: pointer;
      margin-top: 1rem;
      transition: transform 0.1s;
    }
    button:active { transform: scale(0.95); }
  </style>
</head>
<body>
  <div class="card">
    <h1>Hello, World!</h1>
    <p>This is a live preview running directly in your AI Toolbox.</p>
    <button onclick="alert('It works!')">Click Me</button>
  </div>
</body>
</html>`

onMounted(() => {
  htmlCode.value = defaultCode
})

const runCode = () => {
  showPreview.value = true
  // Wait for transition/render
  setTimeout(() => {
    if (previewFrame.value) {
      const blob = new Blob([htmlCode.value], { type: 'text/html' })
      previewFrame.value.src = URL.createObjectURL(blob)
    }
  }, 100)
}

const closePreview = () => {
  showPreview.value = false
  // Clean up iframe src to stop scripts/audio etc.
  setTimeout(() => {
    if (previewFrame.value) {
      previewFrame.value.src = 'about:blank'
    }
  }, 300)
}
</script>
