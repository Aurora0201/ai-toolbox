import { createRouter, createWebHashHistory } from 'vue-router'

const routes = [
  {
    path: '/',
    name: 'Home',
    component: () => import('../features/home/HomeView.vue')
  },
  {
    path: '/chat',
    name: 'Chat',
    component: () => import('../features/chat/ChatView.vue')
  },
  {
    path: '/translator',
    name: 'Translator',
    component: () => import('../features/translator/TranslatorView.vue')
  },
  {
    path: '/dashboard',
    name: 'Dashboard',
    component: () => import('../features/dashboard/DashboardView.vue')
  },
  {
    path: '/models',
    name: 'Models',
    component: () => import('../features/models/ModelsView.vue')
  },
  {
    path: '/settings',
    name: 'Settings',
    component: () => import('../features/settings/SettingsView.vue')
  }
]

const router = createRouter({
  history: createWebHashHistory(),
  routes
})

export default router