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
    name: 'zh-CN',
    displayName: '简体中文'
  },
  {
    name: 'en-US',
    displayName: 'English'
  }
]

export type MessageSchema = typeof enUS | typeof zhCN

export const i18n = () => {
  const appStore = useAppStore()
  const i18n = createI18n<MessageSchema>({
    legacy: false,
    globalInjection: true,
    locale: appStore.app.lang || 'zh-CN',
    fallbackLocale: 'en-US',
    silentTranslationWarn: true,
    messages
  })
  return i18n
}
