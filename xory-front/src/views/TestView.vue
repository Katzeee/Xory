<template>
  <div>
    <v-btn color="success" class="mt-12" @click="overlay = !overlay"> Show Overlay </v-btn>

    <div v-if="overlay" class="popup"></div>
  </div>

  <!-- <div class="column-root-container">
    <v-overlay v-model="overlay" contained class="align-center justify-center">
      <v-btn color="success" @click="overlay = false"> Hide Overlay </v-btn>
    </v-overlay>
  </div> -->
</template>

<script setup lang="ts">
import { ref } from 'vue'

const overlay = ref(true)
</script>

<style scoped lang="scss">
.v-overlay {
  width: 300px;
  height: 300px;
}

.popup {
  position: relative;
  left: 0px;
  width: 300px;
  height: 300px;
  background-color: aqua;
}
</style>
