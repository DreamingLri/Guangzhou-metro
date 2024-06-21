<script setup lang="ts">
import {onMounted, ref} from "vue";

const showSessionId = ref('')
const src = ref('')

onMounted(()=>{
  showSessionId.value = window.location.search
  src.value = '../../../public/matro.html'
})
</script>

<template>
  <div style="width: 100%; height: 100vh">
    <iframe :src="src" frameborder="0" style="height: 100vh; width: 100%"></iframe>
  </div>
</template>

<style scoped>

</style>