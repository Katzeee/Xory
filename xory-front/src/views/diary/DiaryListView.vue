<template>
  <div class="">
    <v-btn style="margin-bottom: 20px" type="primary" size="default" @click="onSearch">List</v-btn>
    <div v-for="(item, index) in diaries" :key="index" class="diary-list-group">
      <v-card class="mx-auto" max-width="344" @click="onDetail(item.id)" hover>
        <v-card-item>
          <v-card-title> {{ item.title }} </v-card-title>
          <v-card-subtitle> {{ item.category }} </v-card-subtitle>
        </v-card-item>
      </v-card>
      <v-card></v-card>
      <!-- <ListHeader v-if="!item.title" size="" :title="item.date" /> -->
    </div>
  </div>
</template>

<script setup lang="ts">
import { type DiaryListRes, diaryList } from '@/api/diary'
import { useUserStore } from '@/stores/user'
import { ref } from 'vue'
import { useRouter } from 'vue-router'
const router = useRouter()
const userStore = useUserStore()

const diaries = ref<DiaryListRes[]>([])

const onSearch = async () => {
  await diaryList({
    uid: userStore.userInfo.uid,
    page_number: 0,
    page_size: 0,
    keywords: '[]',
    category: '[]'
  }).then((data) => {
    diaries.value = data.value!
  })
}

onSearch()

const onDetail = (id: number) => {
  router.push(`/diary/detail/${id}`)
}
</script>

<style scoped lang="scss">
.diary-list-group {
  margin-bottom: 10px;
}
</style>
