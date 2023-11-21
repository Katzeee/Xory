<template>
  <div class="row">
    <div class="left">{{ name }}</div>
    <div class="right" ref="rightRef">
      <slot name="content"> </slot>
      <v-overlay
        scroll-strategy="close"
        location="start top"
        origin="overlap"
        activator="parent"
        transition="scale-transition"
        location-strategy="connected"
        scrim="transparent"
        :max-width="rightWidth - 2"
      >
        <v-card elevation="10">
          <slot name="overlay"></slot>
        </v-card>
      </v-overlay>
    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted } from 'vue'
import { ref } from 'vue'
// 定义并初始化 props
const props = defineProps({
  name: {
    type: String,
    required: true as const
  }
})
const rightRef = ref<HTMLElement | null>(null)
const rightWidth = ref(0)

onMounted(() => {
  rightWidth.value = rightRef.value?.offsetWidth as number
  window.addEventListener('resize', () => {
    rightWidth.value = rightRef.value?.offsetWidth as number
  })
})

// 使用 `ref` 将 `props.message` 转化为响应式引用
</script>

<style scoped lang="scss">
.row {
  display: flex;
  align-items: center;
  min-height: 24px;
  .left {
    flex: 0 1;
    min-width: 80px;
    color: var(--vt-c-text-light-2);
  }
  .right {
    flex: 1 0;
    height: 100%;
  }
}

.v-card {
  padding: 8px;
  // margin-right: 10px;
}
</style>
