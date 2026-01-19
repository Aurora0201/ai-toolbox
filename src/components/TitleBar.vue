<template>
  <div
    data-tauri-drag-region
    class="h-8 bg-background-surface border-b border-border select-none flex justify-between items-center absolute top-0 left-0 right-0 z-50 text-text-main"
  >
    <div class="ml-4 text-[13px] font-medium pointer-events-none">
      AI Toolbox
    </div>
    <div class="flex h-full">
      <button
        class="flex items-center justify-center w-[46px] h-full bg-transparent border-none cursor-pointer outline-none text-text-main hover:bg-background-element transition-colors duration-200"
        @click="minimize"
      >
        <Minus class="w-2.5 h-2.5" />
      </button>
      <button
        class="flex items-center justify-center w-[46px] h-full bg-transparent border-none cursor-pointer outline-none text-text-main hover:bg-background-element transition-colors duration-200"
        @click="toggleMaximize"
      >
        <Square class="w-2.5 h-2.5" />
      </button>
      <button
        class="flex items-center justify-center w-[46px] h-full bg-transparent border-none cursor-pointer outline-none text-text-main hover:bg-danger hover:text-white transition-colors duration-200"
        @click="close"
      >
        <X class="w-3 h-3" />
      </button>
    </div>
  </div>
</template>

<script setup>
import { getCurrentWindow } from '@tauri-apps/api/window';
import { Minus, Square, X } from 'lucide-vue-next';

const appWindow = getCurrentWindow();

/**
 * Minimizes the window
 */
function minimize() {
  appWindow.minimize();
}

/**
 * Toggles maximize/unmaximize
 */
async function toggleMaximize() {
  await appWindow.toggleMaximize();
}

/**
 * Closes the window
 */
function close() {
  appWindow.close();
}
</script>

