<script setup>
import TitleBar from "./components/TitleBar.vue";
import AppSidebar from "./components/AppSidebar.vue";
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
  </div>
</template>

<style>
/* Global resets handled here or in main css */
body {
  margin: 0;
  padding: 0;
  overflow: hidden; /* Prevent body scroll, handle in content area */
  background-color: #f6f6f6;
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
}

@media (prefers-color-scheme: dark) {
  body {
    background-color: #2f2f2f;
    color: #f6f6f6;
  }
}
</style>

<style scoped>
.app-layout {
  display: flex;
  flex-direction: column;
  height: 100vh;
  width: 100vw;
}

.main-container {
  display: flex;
  flex: 1;
  margin-top: 32px; /* Height of TitleBar */
  height: calc(100vh - 32px);
  overflow: hidden;
}

.content-area {
  flex: 1;
  overflow-y: auto;
  padding: 0;
  background-color: var(--content-bg, #ffffff);
}

@media (prefers-color-scheme: dark) {
  .content-area {
    background-color: #1a1a1a;
  }
}
</style>