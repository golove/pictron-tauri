<script setup lang="ts">
import masonry from '../components/gMasonry.vue'
import gCard from '../components/gCard.vue'
import { useRouter } from 'vue-router'
import { storeToRefs } from 'pinia';
import { useCounterStore }from '../stores/counter';
const store = useCounterStore()
const { cols,photos } = storeToRefs(store)

const router = useRouter()
function click(title: string, n: number) {
  router.push(`/bigView/${title}/${n}`)
}

</script>

<template>
  
    <!-- <div class="masonry">
        <card v-for="img in imgs" :key="img.src" :img="img"  />
    </div> -->

    <masonry :cols="cols" :gap="10">
        <gCard v-for="(img,i) in photos.srcs" :aspectRatio="img.aspect_ratio" :key="img.src"
         :src="img.src" :width="230" absolute @click="click(photos.title, i)" />
    </masonry>
    

</template>
<style>

</style>
