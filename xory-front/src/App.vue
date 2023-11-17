<template>
  <!-- <header>
    <img alt="Vue logo" class="logo" src="@/assets/logo.svg" width="125" height="125" />

    <div class="wrapper">
      <HelloWorld msg="You did it!" />

      <nav>
        <RouterLink to="/">Home</RouterLink>
        <RouterLink to="/login">Login</RouterLink>
      </nav>
    </div>
  </header> -->
  <el-config-provider :locale="locale">
    <RouterView />
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
