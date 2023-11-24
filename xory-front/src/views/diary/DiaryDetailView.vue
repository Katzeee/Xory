<template>
  <v-container class="column-root-container">
    <v-app-bar color="primary" density="compact">
      <template v-slot:prepend>
        <v-btn icon @click="router.go(-1)">
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
        <attribute-item :name="t('diary.detail.date')">
          <template v-slot:content>
            <div class="attribute-content">{{ diary.date }}</div>
          </template>
          <template v-slot:overlay>
            <v-date-picker v-model="dateRef" hide-header show-adjacent-months></v-date-picker>
          </template>
        </attribute-item>
        <attribute-item :name="t('diary.detail.tag')">
          <template v-slot:content>
            <div class="attribute-content">
              {{ diary.tags?.toString() }}
            </div>
          </template>
          <template v-slot:overlay>
            <v-chip-group selected-class="text-primary" multiple v-model="diary.tags">
              <v-chip v-for="(tag, id) in userStore.userInfo.tags" :key="id" label>
                {{ tag.name }}
              </v-chip>
            </v-chip-group>
          </template>
        </attribute-item>
        <attribute-item :name="t('diary.detail.weather')">
          <template v-slot:content>
            <div class="attribute-content">{{ diary.weather?.toString() }}</div>
          </template>
          <template v-slot:overlay>
            <div class="time-picker">
              <v-btn icon="mdi-chevron-up" variant="plain" size="small" flat></v-btn>
              <div>12</div>
              <v-btn icon="mdi-chevron-down" variant="plain" size="small" flat></v-btn>
            </div>
          </template>
        </attribute-item>
        <attribute-item :name="t('diary.detail.mood')" :close-on-content-click="true">
          <template v-slot:content>
            <div class="attribute-content">
              {{ moodIndex != null ? moods[moodIndex]['name'] : 'Empty' }}
            </div>
          </template>
          <template v-slot:overlay>
            <v-chip-group class="text-center" v-model="moodIndex" selected-class="text-primary">
              <v-chip v-for="(mood, id) in moods" :key="id" label>
                <v-icon :color="mood['color']" start :icon="mood['icon']"></v-icon>
                {{ mood['name'] }}
              </v-chip>
            </v-chip-group>
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
import { watch } from 'vue'
declare let AMap: any
const route = useRoute()
const router = useRouter()
const { t } = useI18n<{ message: MessageSchema }>({ useScope: 'global' })
const appStore = useAppStore()
const userStore = useUserStore()
interface DiaryDetailDisp extends DiaryDetailRes {
  showMap?: boolean
}

const dateRef = ref(new Date('2022-11-01'))

const moods = [
  { name: 'elated', icon: 'mdi-emoticon-cool', color: 'green-lighten-2' },
  { name: 'content', icon: 'mdi-emoticon-happy-outline', color: 'light-blue-lighten-2' },
  { name: 'neutral', icon: 'mdi-emoticon-neutral', color: 'blue-grey-lighten-2' },
  { name: 'displeased', icon: 'mdi-emoticon-cry-outline', color: 'yellow-darken-3' },
  { name: 'miserable', icon: 'mdi-emoticon-dead', color: 'red-lighten-2' }
]
const moodIndex = ref<number | null>(null)

const createModifyFlag = ref(true)
const diary = ref<DiaryDetailDisp>({})
const requestDiaryDetail = async () => {
  await diaryDetail({ did: Number(route.params.id) }).then((data) => {
    let { latitude, longitude, date, date_create, date_modify } = data.value!
    diary.value = data.value!
    diary.value.date = new Date(date as string)
    dateRef.value = diary.value.date
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
watch(
  () => diary.value.tags as number[],
  (newValue, oldValue) => {
    if (newValue.length === 0) {
      ;(diary.value.tags as number[]).push(0)
      return
    }
    if (newValue.length > 1) {
      const index = (diary.value.tags as number[]).indexOf(0)
      if (index !== -1) {
        ;(diary.value.tags as number[]).splice(index, 1)
      }
    }
  }
)

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

.v-date-picker {
  width: auto;
  :deep(.v-date-picker-years__content) {
    padding-inline: 0;
  }
  :deep(.v-date-picker-controls) {
    padding: 0;
  }
  :deep(.v-date-picker-month) {
    min-width: 270px;
    padding: 0 0 12px;
  }
  :deep(.v-date-picker-controls) {
    .v-spacer {
      flex: 0 1;
    }
  }
  :deep(.v-date-picker-month__days) {
    // grid-template-columns: repeat(7, 25px) !important;
    // grid-template-rows: repeat(6, 25px) !important;
    flex: 0 1 0 !important;
  }
  :deep(.v-date-picker-month__day) {
    width: 38px !important;
    height: 35px !important;
    button {
      --v-btn-height: 14px !important;
      width: calc(var(--v-btn-height) + 13px);
      height: calc(var(--v-btn-height) + 13px);
    }
  }
}

pre {
  flex-grow: 1;
}

.attribute-content {
  display: flex;
  padding: 8px;
  align-items: center;
  margin-right: 20px;
  &:hover {
    background: rgba(55, 53, 47, 0.08);
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

.time-picker {
  display: flex;
  flex-direction: column;
}
</style>
