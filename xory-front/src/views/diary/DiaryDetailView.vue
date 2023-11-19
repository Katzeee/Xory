<template>
  <div class="column-root-container">
    <v-btn @click="onBack">Back</v-btn>
    <!-- <div> -->
    <v-textarea
      rows="1"
      auto-grow
      v-model="diary.title"
      variant="underlined"
      class="title"
    ></v-textarea>
    <!-- </div> -->
    <div class="attributes-group">
      <attribute-item :name="t('diary.detail.tag')">
        <template v-slot:content>
          <div class="attribute-content">{{ diary.category?.toString() }}</div>
        </template>
      </attribute-item>
      <attribute-item :name="t('diary.detail.date')">
        <template v-slot:content>
          <div class="attribute-content">{{ diary.date?.toString() }}</div>
        </template>
      </attribute-item>
      <attribute-item :name="t('diary.detail.weather')">
        <template v-slot:content>
          <div class="attribute-content">{{ diary.weather?.toString() }}</div>
        </template>
      </attribute-item>
      <div>{{ 0 }}</div>
    </div>
    <v-divider></v-divider>
    <div>
      <div>
        {{ diary.content }}
      </div>
      <div>
        {{ diary.date_modify }}
      </div>
    </div>

    <div class="map-container" v-if="diary.showMap">
      <el-amap ref="mapRef" :center="center" :zoom="zoom" @init="init" />
    </div>
    <div v-else>No map info.</div>
    <!-- <pre style="white-space: pre-wrap">{{ JSON.stringify(diary, null, 2) }}</pre> -->
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { diaryDetail, type DiaryDetailRes } from '@/api/diary'
import { ElAmap } from '@vuemap/vue-amap'
import type { MessageSchema } from '@/i18n'
import { useI18n } from 'vue-i18n'
import AttributeItem from './AttributeItem.vue'
declare let AMap: any
const route = useRoute()
const router = useRouter()
const { t } = useI18n<{ message: MessageSchema }>({ useScope: 'global' })

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
.title {
  flex: 0 1 auto;
  justify-self: flex-start;
  :deep(textarea) {
    font-size: 30px;
    padding: 6px;
  }
  :deep(.v-field__outline)::before {
    display: none;
  }
}
button {
  margin-bottom: 20px;
}

.v-divider {
  margin: 20px 0px;
}
.map-container {
  flex: 1;
  max-height: 300px;
  min-height: 100px;
}

pre {
  flex-grow: 1;
}

.attribute-content {
  flex: 1 0;
  padding: 8px;
  &:hover {
    background: rgba(55, 53, 47, 0.08);
  }
}
</style>
