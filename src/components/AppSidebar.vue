<template>
  <aside class="sidebar">
    <div class="logo">
      AI Toolbox
    </div>
    <nav>
      <router-link
        to="/"
        class="nav-item"
        active-class="active"
      >
        Home
      </router-link>
      <router-link
        to="/settings"
        class="nav-item"
        active-class="active"
      >
        Settings
      </router-link>
    </nav>
  </aside>
</template>

<script setup>
// Sidebar navigation component
</script>

<style scoped>
.sidebar {
  width: 250px;
  background-color: #2f3241;
  color: white;
  display: flex;
  flex-direction: column;
  height: 100%;
}

.logo {
  padding: 20px;
  font-size: 1.5em;
  font-weight: bold;
  text-align: center;
  border-bottom: 1px solid #3e4255;
}

nav {
  display: flex;
  flex-direction: column;
  padding: 10px;
}

.nav-item {
  padding: 15px;
  color: #a0a0a0;
  text-decoration: none;
  border-radius: 8px;
  margin-bottom: 5px;
  transition: background-color 0.2s, color 0.2s;
}

.nav-item:hover {
  background-color: #3e4255;
  color: white;
}

.nav-item.active {
  background-color: #646cff;
  color: white;
}
</style>
