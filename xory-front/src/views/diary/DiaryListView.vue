<template>
  <div class="">
    <el-button style="margin-bottom: 20px" type="primary" size="default" @click="onSearch"
      >List</el-button
    >
    <div v-for="(item, index) in diaries" :key="index" class="diary-list-group">
      <v-card class="mx-auto" max-width="344" hover href="https://www.baidu.com">
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
import { type DiaryListRes, listDiaries } from '@/api/home'
import { useUserStore } from '@/stores/user'
import { ref } from 'vue'
const userStore = useUserStore()

const diaries = ref<DiaryListRes[]>([])

const onSearch = async () => {
  await listDiaries({
    uid: userStore.userInfo.uid,
    page_number: 0,
    page_size: 0,
    keywords: '[]',
    category: '[]'
  }).then((data) => {
    console.log(data)
    diaries.value = data.value!
  })
}
</script>

<style scoped lang="scss">
.diary-list-group {
  margin-bottom: 10px;
}
</style>
