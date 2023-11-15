import { ref, computed } from 'vue'
import { defineStore } from 'pinia'

export enum LangType {
  enUS,
  zhCN
}

interface App {
  lang?: LangType
}

export const useAppStore = defineStore('app', () => {
  const app: App = {
    lang: LangType.enUS
  }

  return { app }
})
