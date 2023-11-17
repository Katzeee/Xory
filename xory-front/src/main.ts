// import '@/assets/main.scss'
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
const vuetify = createVuetify({})
app.use(vuetify)

app.mount('#app')
