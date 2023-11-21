<template>
  <div class="column-root-container">
    <v-btn id="here" color="success" class="mt-12"> Show Overlay </v-btn>

    <v-overlay activator="#here" location-strategy="connected">
      <div class="popup"></div>
      <!-- <v-card variant="flat"> 123123 </v-card> -->
    </v-overlay>
  </div>

  <!-- <div class="column-root-container">
    <v-overlay v-model="overlay" contained class="align-center justify-center">
      <v-btn color="success" @click="overlay = false"> Hide Overlay </v-btn>
    </v-overlay>
  </div> -->
</template>

<script setup lang="ts"></script>

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
