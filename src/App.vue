<script setup>
import TitleBar from "./components/TitleBar.vue";
import AppSidebar from "./components/AppSidebar.vue";
import ToastContainer from "./components/ToastContainer.vue";
</script>

<template>
  <div class="app-layout">
    <TitleBar />
    <div class="main-container">
      <AppSidebar />
      <main class="content-area">
        <router-view />
      </main>
    </div>
    <ToastContainer position="top-right" />
  </div>
</template>

<style scoped>
.app-layout {
  display: flex;
  flex-direction: column;
  height: 100vh;
  width: 100vw;
  background-color: var(--bg-app);
  overflow: hidden; /* Critical: prevent body scroll */
}

.main-container {
  display: flex;
  flex: 1;
  /* TitleBar is usually around 30-32px. If it's fixed/absolute, we might need padding. 
     If it's in the flex flow, we don't. Assuming TitleBar is in flex flow or fixed?
     Let's check TitleBar style. It is fixed. So we need margin-top.
  */
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
