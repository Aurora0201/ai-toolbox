<script setup>
import { onMounted, ref } from 'vue'
import AppSidebar from './components/AppSidebar.vue'
import TitleBar from './components/TitleBar.vue'
import ToastContainer from './components/ToastContainer.vue'
import ConfirmDialog from './components/common/ConfirmDialog.vue'
import { useSettingsStore } from './store/settings'
import { useConfirm } from './composables/useConfirm'
import { Loader2 } from 'lucide-vue-next'

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
    class="flex flex-col h-screen w-screen bg-background-app overflow-hidden relative rounded-lg"
  >
    <TitleBar />
    <div class="flex flex-1 mt-8 h-[calc(100vh-32px)] overflow-hidden">
      <AppSidebar />
      <main class="flex-1 overflow-y-auto p-0 bg-background-app relative">
        <router-view v-slot="{ Component }">
          <transition
            enter-active-class="transition-opacity duration-200 ease-out"
            enter-from-class="opacity-0"
            leave-active-class="transition-opacity duration-200 ease-in"
            leave-to-class="opacity-0"
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
    class="flex justify-center items-center h-screen bg-background-app"
  >
    <Loader2 class="w-10 h-10 text-primary animate-spin" />
  </div>
</template>

