<template>
  <div class="root">
    <div>
      <v-btn @click="onBack">Back</v-btn>
    </div>
    <pre style="white-space: pre-wrap">{{ JSON.stringify(diary, null, 2) }}</pre>
    <div class="map-container" v-if="diary.showMap">
      <el-amap ref="mapRef" :center="center" :zoom="zoom" @init="init" />
    </div>
    <div v-else>No map info.</div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { diaryDetail, type DiaryDetailRes } from '@/api/diary'
import { ElAmap } from '@vuemap/vue-amap'
declare let AMap: any
const route = useRoute()
const router = useRouter()

interface DiaryDetailDisp extends DiaryDetailRes {
  showMap?: boolean
}

const onBack = () => {
  router.go(-1)
}

const diary = ref<DiaryDetailDisp>({})

const requestDiaryDetail = async () => {
  await diaryDetail({ id: +route.params.id }).then((data) => {
    let { latitude, longitude } = data.value!
    diary.value = data.value!
    diary.value.showMap = latitude != undefined && longitude != undefined
  })
}

requestDiaryDetail()

const zoom = ref(1)
let center = ref([0, 0])
const init = (map: any) => {
  center.value = [diary.value.longitude!, diary.value.latitude!]
  const marker = new AMap.Marker({
    position: center.value
  })
  map.add(marker)
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
  .map-container {
    flex: 1;
    max-height: 300px;
  }
}
</style>
