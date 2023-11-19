<template>
  <div class="row">
    <div class="left">{{ name }}</div>
    <slot class="right" name="content"></slot>
    <!-- <div class="right">{{ content }}</div> -->
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
// 定义并初始化 props
const props = defineProps({
  name: {
    type: String,
    required: true as const
  },
  content: {
    type: String
  }
})

// 使用 `ref` 将 `props.message` 转化为响应式引用
</script>

<style scoped lang="scss">
.row {
  display: flex;
  .left {
    flex: 0 1;
    height: 24px;
    min-width: 80px;
    color: var(--vt-c-text-light-2);
    padding: 8px;
  }
  .right {
  }
}
</style>
