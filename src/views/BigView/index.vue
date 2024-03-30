<script setup lang="ts">
// import Photo from './photoBig.vue'
import Photo from './photoView.vue'
import { useCounterStore } from '@/stores/counter'
const store = useCounterStore()

</script>

<template>
  <Photo :photos="store.photos.srcs"/>
  <!-- <div>photos</div> -->
</template>
