// import '@/assets/main.scss'
import { createApp } from 'vue'
import { createPinia } from 'pinia'
import ElementPlus from 'element-plus'
import 'element-plus/dist/index.css'
import App from '@/App.vue'
import { router } from '@/router'
import { i18n } from '@/i18n'

const app = createApp(App)

app.use(ElementPlus)
app.use(createPinia())
app.use(router())

app.use(i18n())

app.mount('#app')
