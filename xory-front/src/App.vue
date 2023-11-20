<template>
  <el-config-provider :locale="locale">
    <v-app>
      <RouterView />
    </v-app>
  </el-config-provider>
</template>

<script setup lang="ts">
import { RouterLink, RouterView } from 'vue-router'
import HelloWorld from './components/HelloWorld.vue'
import { useAppStore, LangType } from '@/stores/app'
import { ref } from 'vue'
// import
import en from 'element-plus/es/locale/lang/en'
import zhCN from 'element-plus/es/locale/lang/zh-cn'
const appStore = useAppStore()
let locale = ref(appStore.app.lang === LangType.zhCN ? zhCN : en)
</script>

<style lang="scss">
// must be here to make scss preprocessor work
// cannot be scoped
</style>
