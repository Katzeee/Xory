<template>
  <div class="root">
    <div>
      <v-btn @click="onBack">Back</v-btn>
    </div>
    <pre style="white-space: pre-wrap">{{ JSON.stringify(diary, null, 2) }}</pre>
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
</script>

<style scoped lang="scss">
.root {
  display: flex;
  flex-direction: column;
  button {
    margin-bottom: 20px;
  }
}
</style>
