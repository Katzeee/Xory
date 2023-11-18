import '@mdi/font/css/materialdesignicons.css'
import { createApp } from 'vue'
import { createPinia } from 'pinia'
import ElementPlus from 'element-plus'
import 'element-plus/dist/index.css'
import App from '@/App.vue'

const app = createApp(App)

app.use(ElementPlus)
app.use(createPinia())

import { router } from '@/router'
app.use(router())

import { i18n } from '@/i18n'
app.use(i18n())

import 'vuetify/styles'
import { createVuetify } from 'vuetify'
import { aliases, mdi } from 'vuetify/iconsets/mdi'
const vuetify = createVuetify({
  icons: {
    defaultSet: 'mdi',
    aliases,
    sets: {
      mdi
    }
  }
})
app.use(vuetify)

import { initAMapApiLoader } from '@vuemap/vue-amap'
import '@vuemap/vue-amap/dist/style.css'
const key = import.meta.env.VITE_AMAP_KEY
const sKey = import.meta.env.VITE_AMAP_SKEY
initAMapApiLoader({
  key: key,
  securityJsCode: sKey
  //Loca:{
  //  version: '2.0.0'
  //} // 如果需要使用loca组件库，需要加载Loca
})

app.mount('#app')
