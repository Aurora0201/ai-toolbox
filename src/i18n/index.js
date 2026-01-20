import { createI18n } from 'vue-i18n'
import en from './locales/en.json'
import zh from './locales/zh.json'

const messages = {
  en,
  zh
}

/**
 * vue-i18n instance configuration.
 * Uses translations from local JSON files.
 */
const i18n = createI18n({
  legacy: false, // Use Composition API
  locale: 'zh', // Default locale
  fallbackLocale: 'en',
  messages,
})

export default i18n
