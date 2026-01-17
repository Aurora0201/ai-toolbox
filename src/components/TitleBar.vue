<template>
  <div
    data-tauri-drag-region
    class="titlebar"
  >
    <div class="title">
      AI Toolbox
    </div>
    <div class="window-controls">
      <button
        class="control-btn minimize"
        @click="minimize"
      >
        <svg
          viewBox="0 0 10 1"
          width="10"
          height="10"
        ><path d="M0 0h10v1H0z" /></svg>
      </button>
      <button
        class="control-btn maximize"
        @click="toggleMaximize"
      >
        <svg
          viewBox="0 0 10 10"
          width="10"
          height="10"
        ><path d="M0 0h10v10H0V0zm1 1v8h8V1H1z" /></svg>
      </button>
      <button
        class="control-btn close"
        @click="close"
      >
        <svg
          viewBox="0 0 10 10"
          width="10"
          height="10"
        ><path
          d="M0 0L10 10M10 0L0 10"
          stroke="currentColor"
          stroke-width="1.2"
        /></svg>
      </button>
    </div>
  </div>
</template>

<script setup>
import { getCurrentWindow } from '@tauri-apps/api/window';

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

<style scoped>
.titlebar {
  height: 32px;
  background: var(--bg-sidebar);
  border-bottom: 2px solid var(--border-color);
  user-select: none;
  display: flex;
  justify-content: space-between;
  align-items: center;
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  z-index: 1000;
  color: var(--text-primary);
}

.title {
  margin-left: 15px;
  font-size: 13px;
  font-weight: 500;
  pointer-events: none; /* Let clicks pass through to drag region */
}

.window-controls {
  display: flex;
  height: 100%;
}

.control-btn {
  background: transparent;
  border: none;
  color: var(--text-primary);
  width: 46px;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  cursor: pointer;
  outline: none;
  transition: background-color 0.2s;
}

.control-btn:hover {
  background-color: var(--bg-hover);
}

.control-btn.close:hover {
  background-color: #e81123;
  color: white;
}

.control-btn svg {
  fill: currentColor;
}
</style>
