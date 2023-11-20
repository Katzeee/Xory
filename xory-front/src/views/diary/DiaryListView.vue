<template>
  <div class="root">
    <!-- <div v-for="(item, index) in diaries" :key="index" class="diary-list-group">
      <v-card class="mx-auto" max-width="344" @click="onDetail(item.id)" hover>
        <v-card-item prepend-icon="mdi-weather-pouring">
          <v-card-title> {{ item.title }} </v-card-title>
          <v-card-subtitle> {{ item.category }} </v-card-subtitle>
        </v-card-item>
      </v-card>
      <v-card></v-card>
    </div> -->
    <v-timeline side="end" class="diary-list-group">
      <v-timeline-item size="small" v-for="(item, index) in diaries" :key="index">
        <template v-if="!item.isHeader && item.showDate" v-slot:opposite>
          {{
            (item.date as Date).toLocaleDateString(appStore.app.lang, {
              day: 'numeric'
            })
          }}
        </template>
        <v-card v-if="!item.isHeader" @click="onDetail(item.did!)" class="diary-item">
          {{ item.title }}
        </v-card>
        <div v-else style="color: brown">
          {{
            getYearMonth(item.date as Date).toLocaleDateString(appStore.app.lang, {
              year: 'numeric',
              month: 'long'
            })
          }}
        </div>
      </v-timeline-item>
    </v-timeline>
    <v-btn style="margin: 16px 8px 0" color="blue-darken-4" size="default" @click="onSearch"
      >List</v-btn
    >
  </div>
</template>

<script setup lang="ts">
import { type DiaryListRes, diaryList } from '@/api/diary'
import { useUserStore } from '@/stores/user'
import { ref } from 'vue'
import { useRouter } from 'vue-router'
import { useAppStore } from '@/stores/app'
const router = useRouter()
const userStore = useUserStore()
const appStore = useAppStore()

interface DiaryListDisp extends DiaryListRes {
  isHeader?: boolean
  showDate?: boolean
}
const diaries = ref<DiaryListDisp[]>([])

const requestDiaryList = async () => {
  await diaryList({
    uid: userStore.userInfo.uid,
    page_number: 0,
    page_size: 0,
    keywords: '[]',
    tags: '[1,2,3,4,5]'
  }).then((data) => {
    diaries.value = data.value!.map((item) => {
      const { date: dateString, ..._ } = item
      const date = new Date(dateString as string)
      return { date: date, header: false, ..._ }
    })
  })
}

const getYearMonth = (date: Date) => {
  return new Date(date.getFullYear(), date.getMonth())
}

const getYearMonthDay = (date: Date) => {
  return new Date(date.getFullYear(), date.getMonth(), date.getDay())
}

const postProcess = () => {
  if (diaries.value.length === 0) {
    return
  }
  diaries.value.splice(0, 0, {
    isHeader: true,
    date: new Date(diaries.value[0].date!)
  })
  for (let i = 1; i < diaries.value.length; i++) {
    let lastDate = diaries.value[i - 1].date as Date
    let curDate = diaries.value[i].date as Date
    if (getYearMonthDay(curDate) !== getYearMonthDay(lastDate)) {
      diaries.value[i].showDate = true
    } else {
      diaries.value[i].showDate = false
    }
    if (getYearMonth(curDate) > getYearMonth(lastDate)) {
      diaries.value.splice(i, 0, {
        isHeader: true,
        date: new Date(diaries.value[i].date!)
      })
      i++
    }
  }
}

const onSearch = async () => {
  await requestDiaryList()
  postProcess()
}

onSearch()

const onDetail = (id: number) => {
  router.push(`/diary/detail/${id}`)
}
</script>

<style scoped lang="scss">
.root {
  display: flex;
  flex-direction: column;
  justify-content: space-around;
  flex: 1;
}

.diary-list-group {
  margin-bottom: 12px;
  overflow-y: scroll;
  :deep(.v-timeline-item__body) {
    justify-self: auto !important;
    padding: 10px;
    margin: 10px;
    overflow: hidden;
  }
  .diary-item {
    padding: 10px;
  }
}
</style>
