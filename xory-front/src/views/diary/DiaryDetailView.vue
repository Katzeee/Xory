<template>
  <div class="root">
    <div>
      <v-btn @click="onBack">Back</v-btn>
    </div>
    <pre style="white-space: pre-wrap">{{ JSON.stringify(diary, null, 2) }}</pre>
    <div class="map-page-container">
      <el-amap :center="center" :zoom="zoom" @init="init" />
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { diaryDetail, type DiaryDetailReq } from '@/api/diary'
const route = useRoute()
const router = useRouter()

const diary = ref<DiaryDetailReq>({
  id: 0
})

diaryDetail({ id: +route.params.id }).then((data) => {
  diary.value = data.value!
})

const onBack = () => {
  router.go(-1)
}

import { ElAmap } from '@vuemap/vue-amap'

const zoom = ref(12)
const center = ref([121.59996, 31.197646])
let map: any
declare let AMap: any
const init = (e: any) => {
  console.log(AMap)

  const marker = new AMap.Marker({
    position: [121.59996, 31.197646]
  })
  e.add(marker)
  map = e
  console.log('map init: ', map)
}
</script>

<style scoped lang="scss">
.root {
  display: flex;
  flex-direction: column;
  flex: 1;
  button {
    margin-bottom: 20px;
  }
  .map-page-container {
    flex: 1;
  }
}
</style>
