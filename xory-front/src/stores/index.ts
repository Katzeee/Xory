import { ref, computed } from 'vue'
import { defineStore } from 'pinia'

interface App {
  lang?: string
}

export const useAppStore = defineStore('app', () => {
  const app: App = {
    lang: 'en-US'
  }

  return { app }
})
