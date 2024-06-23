<script setup lang="ts">
import {onMounted, ref} from "vue";

const showSessionId = ref('')
const src_guangzhou = ref('')
const src_foshan = ref('')

onMounted(()=>{
  showSessionId.value = window.location.search
  src_guangzhou.value = '../../../public/metro_guangzhou.html'
  src_foshan.value = '../../../public/metro_foshan.html'
})

defineProps({
  map_switch: Boolean
})
</script>

<template>
  <div style="width: 100%; height: 100vh">
    <iframe :src="map_switch ? src_guangzhou : src_foshan" frameborder="0" style="height: 100vh; width: 100%"></iframe>
  </div>
</template>

<style scoped>

</style>