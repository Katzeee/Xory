<template>
  <div class="">
    <el-button type="primary" size="default" @click="onSearch"></el-button>
    <div v-for="(item, index) in diaries" :key="index" class="diary-list-group">
      <!-- <ListHeader v-if="!item.title" size="" :title="item.date" /> -->
      <div>
        <span>{{ index }}</span> :
        <span>{{ item.title }}</span>
      </div>
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
  overflow: auto;
}
</style>
