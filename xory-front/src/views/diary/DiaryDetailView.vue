<template>
  <v-container class="column-root-container">
    <v-app-bar color="primary" density="compact">
      <template v-slot:prepend>
        <v-btn icon @click="onBack">
          <v-icon icon="mdi-arrow-left"></v-icon>
        </v-btn>
      </template>
    </v-app-bar>
    <v-main>
      <v-textarea
        rows="1"
        auto-grow
        v-model="diary.title"
        variant="underlined"
        class="title"
        hide-details="auto"
      ></v-textarea>
      <div class="attributes-group">
        <attribute-item :name="t('diary.detail.tag')">
          <template v-slot:content>
            <div class="attribute-group">
              <div class="attribute-content">
                {{ diary.tags?.toString() }}
              </div>
              <v-overlay
                v-model="active"
                scroll-strategy="close"
                contained
                location="bottom"
                origin="auto"
                activator="parent"
                location-strategy="connected"
              >
                <v-card class="attribute-card">
                  <v-chip-group selected-class="text-primary" column multiple v-model="diary.tags">
                    <v-chip v-for="(tag, id) in userStore.userInfo.tags" :key="id" label>{{
                      tag.name
                    }}</v-chip>
                  </v-chip-group>
                </v-card>
              </v-overlay>
            </div>
          </template>
        </attribute-item>
        <attribute-item :name="t('diary.detail.date')">
          <template v-slot:content>
            <div class="attribute-group">
              <div class="attribute-content">{{ diary.date }}</div>
            </div>
          </template>
        </attribute-item>
        <attribute-item :name="t('diary.detail.weather')">
          <template v-slot:content>
            <div class="attribute-group" @click="console.log(diary.tags)">
              <div class="attribute-content">{{ diary.weather?.toString() }}</div>
            </div>
          </template>
        </attribute-item>
        <attribute-item :name="t('diary.detail.mood')">
          <template v-slot:content>
            <div class="attribute-group">
              <div class="attribute-content">{{ diary.mood != null ? diary.mood : 'Unknown' }}</div>
              <v-card class="attribute-card">
                <v-chip class="ma-2" color="pink" label>
                  <v-icon start icon="mdi-label"></v-icon>
                  Tags
                </v-chip></v-card
              >
            </div>
          </template>
        </attribute-item>
      </div>
      <v-divider></v-divider>
      <div class="content-group">
        <v-textarea
          rows="1"
          auto-grow
          v-model="diary.content"
          variant="plain"
          class="diary-content"
          density="comfortable"
          hide-details="auto"
        ></v-textarea>
        <div class="create-modify-time" @click="createModifyFlag = !createModifyFlag">
          <span v-if="createModifyFlag" class="label">{{ t('diary.detail.created') }}</span>
          <span v-else class="label">{{ t('diary.detail.modified') }}</span>
          <span v-if="createModifyFlag">{{ diary.date_create }}</span>
          <span v-else>{{ diary.date_modify }}</span>
        </div>
      </div>
    </v-main>
    <div class="map-container" v-if="diary.showMap">
      <el-amap ref="mapRef" :center="center" :zoom="zoom" @init="init" />
    </div>
    <div v-else>No map info.</div>
  </v-container>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { diaryDetail, type DiaryDetailRes } from '@/api/diary'
import { ElAmap } from '@vuemap/vue-amap'
import type { MessageSchema } from '@/i18n'
import { useI18n } from 'vue-i18n'
import AttributeItem from './AttributeItem.vue'
import { useAppStore } from '@/stores/app'
import { useUserStore } from '@/stores/user'
import { computed } from 'vue'
declare let AMap: any
const route = useRoute()
const router = useRouter()
const { t } = useI18n<{ message: MessageSchema }>({ useScope: 'global' })
const appStore = useAppStore()
const userStore = useUserStore()

interface DiaryDetailDisp extends DiaryDetailRes {
  showMap?: boolean
}

const active = ref(false)
const onBack = () => {
  router.go(-1)
}

const createModifyFlag = ref(true)
const diary = ref<DiaryDetailDisp>({})
const requestDiaryDetail = async () => {
  await diaryDetail({ did: Number(route.params.id) }).then((data) => {
    let { latitude, longitude, date, date_create, date_modify } = data.value!
    diary.value = data.value!
    diary.value.date = new Date(date as string)
    diary.value.date_create = new Date(date_create as string).toLocaleString(appStore.app.lang, {
      year: 'numeric',
      month: 'short',
      day: '2-digit',
      hour: '2-digit',
      minute: '2-digit'
    })
    diary.value.date_modify = new Date(date_modify as string).toLocaleString(appStore.app.lang, {
      year: 'numeric',
      month: 'short',
      day: '2-digit',
      hour: '2-digit',
      minute: '2-digit'
    })
    diary.value.showMap = latitude != undefined && longitude != undefined
  })
}

requestDiaryDetail()

const zoom = ref(4)
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
  margin-bottom: 12px;
  :deep(textarea) {
    font-size: 30px;
  }
  :deep(.v-field__outline)::before {
    display: none;
  }
}

.v-divider {
  margin-top: 20px;
}
.map-container {
  flex: 1;
  max-height: 200px;
  min-height: 100px;
}

pre {
  flex-grow: 1;
}

.attribute-group {
  flex: 1 0;
  position: relative;
  // min-height: 42px;
  .attribute-content {
    padding: 8px;
    &:hover {
      background: rgba(55, 53, 47, 0.08);
    }
  }
  .attribute-card {
    padding: 10px;
  }
}

.content-group {
  display: flex;
  flex-direction: column;
  padding: 0px 10px 10px 10px;
  .diary-content {
    font-size: 18px;
    line-height: 3;
    height: auto;
    color: #333;
    margin: 10px 0;
  }

  .create-modify-time {
    align-self: flex-end;
    padding: 0px 10px 10px 0px;
    .label {
      color: rgba(var(--v-border-color), var(--v-disabled-opacity));
    }
    &:hover {
      cursor: default;
    }
  }
}
</style>
