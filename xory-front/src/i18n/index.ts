import { createI18n } from 'vue-i18n'
import { useAppStore } from '@/stores'

import enUS from './data/en-US.json'
import zhCN from './data/zh-CN.json'

const messages = {
  'zh-CN': { ...zhCN },
  'en-US': { ...enUS }
}

interface langInfo {
  name: string
  displayName: string
}

export const langList: langInfo[] = [
  {
    k: 'zh-CN',
    v: '简体中文'
  },
  {
    k: 'en-US',
    v: 'English'
  }
]

export type MessageSchema = typeof enUS | typeof zhCN

const appStore = useAppStore()
export const i18n = createI18n<MessageSchema>({
  legacy: false,
  globalInjection: true,
  locale: appStore.app.lang || 'zhCN',
  fallbackLocale: 'enUS',
  silentTranslationWarn: true,
  messages
})
