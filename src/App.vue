<script setup>
import { onMounted, ref } from 'vue'
import AppSidebar from './components/AppSidebar.vue'
import TitleBar from './components/TitleBar.vue'
import ToastContainer from './components/ToastContainer.vue'
import ConfirmDialog from './components/common/ConfirmDialog.vue'
import { useSettingsStore } from './store/settings'
import { useConfirm } from './composables/useConfirm'

const settings = useSettingsStore()
const { state: confirmState, onConfirm, onCancel } = useConfirm()
const isReady = ref(false)

onMounted(async () => {
  settings.init()
  await settings.syncToBackend()
  isReady.value = true
})
</script>

<template>
  <div
    v-if="isReady"
    class="app-layout"
  >
    <TitleBar />
    <div class="main-container">
      <AppSidebar />
      <main class="content-area">
        <router-view v-slot="{ Component }">
          <transition
            name="fade"
            mode="out-in"
          >
            <component :is="Component" />
          </transition>
        </router-view>
      </main>
    </div>
    <ToastContainer />
    <ConfirmDialog
      :show="confirmState.show"
      :title="confirmState.title"
      :message="confirmState.message"
      :confirm-text="confirmState.confirmText"
      :cancel-text="confirmState.cancelText"
      @confirm="onConfirm"
      @cancel="onCancel"
    />
  </div>
  <div
    v-else
    class="loading-screen"
  >
    <div class="loading-spinner" />
  </div>
</template>

<style>
/* Global Transition and Layout Styles */
.loading-screen {
  display: flex;
  justify-content: center;
  align-items: center;
  height: 100vh;
  background-color: var(--bg-app);
}

.loading-spinner {
  width: 40px;
  height: 40px;
  border: 4px solid var(--bg-hover);
  border-top: 4px solid var(--primary-color);
  border-radius: 50%;
  animation: spin 1s linear infinite;
}

@keyframes spin {
  0% { transform: rotate(0deg); }
  100% { transform: rotate(360deg); }
}

.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.2s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}
</style>

<style scoped>
.app-layout {
  display: flex;
  flex-direction: column;
  height: 100vh;
  width: 100vw;
  background-color: var(--bg-app);
  overflow: hidden; /* Critical: prevent body scroll */
  position: relative;
  border-radius: var(--radius-md);
}

.main-container {
  display: flex;
  flex: 1;
  /* TitleBar is fixed at 32px height */
  margin-top: 32px; 
  height: calc(100vh - 32px);
  overflow: hidden;
}

.content-area {
  flex: 1;
  overflow-y: auto; /* Allow scrolling here */
  padding: 0;
  background-color: var(--bg-app);
  position: relative;
}
</style>
