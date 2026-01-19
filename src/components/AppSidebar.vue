<template>
  <aside class="w-60 bg-background-surface border-r border-border flex flex-col h-full shrink-0">
    <div class="p-4 px-5 text-base font-semibold text-text-main border-b border-border flex items-center gap-2 tracking-tight">
      AI Toolbox
    </div>
    <nav class="p-4 px-2 flex flex-col gap-1">
      <router-link
        to="/"
        class="px-3 py-2 text-text-sub rounded-md text-sm font-medium transition-colors flex items-center gap-2.5 hover:bg-background-element hover:text-text-main"
        active-class="bg-primary text-white shadow-sm hover:bg-primary hover:text-white"
      >
        <Home class="w-[18px] h-[18px]" />
        Home
      </router-link>
      <router-link
        to="/chat"
        class="px-3 py-2 text-text-sub rounded-md text-sm font-medium transition-colors flex items-center gap-2.5 hover:bg-background-element hover:text-text-main"
        active-class="bg-primary text-white shadow-sm hover:bg-primary hover:text-white"
      >
        <MessageSquare class="w-[18px] h-[18px]" />
        Chat
      </router-link>
      <router-link
        to="/dashboard"
        class="px-3 py-2 text-text-sub rounded-md text-sm font-medium transition-colors flex items-center gap-2.5 hover:bg-background-element hover:text-text-main"
        active-class="bg-primary text-white shadow-sm hover:bg-primary hover:text-white"
      >
        <LayoutDashboard class="w-[18px] h-[18px]" />
        Dashboard
      </router-link>
      <router-link
        to="/models"
        class="px-3 py-2 text-text-sub rounded-md text-sm font-medium transition-colors flex items-center gap-2.5 hover:bg-background-element hover:text-text-main"
        active-class="bg-primary text-white shadow-sm hover:bg-primary hover:text-white"
      >
        <Box class="w-[18px] h-[18px]" />
        Models
      </router-link>
      <router-link
        to="/settings"
        class="px-3 py-2 text-text-sub rounded-md text-sm font-medium transition-colors flex items-center gap-2.5 hover:bg-background-element hover:text-text-main"
        active-class="bg-primary text-white shadow-sm hover:bg-primary hover:text-white"
      >
        <Settings class="w-[18px] h-[18px]" />
        Settings
      </router-link>
    </nav>
  </aside>
</template>

<script setup>
import { Home, MessageSquare, LayoutDashboard, Box, Settings } from 'lucide-vue-next';
</script>

