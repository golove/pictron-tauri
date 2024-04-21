<script setup lang="ts">
import { emit, listen ,type Event} from '@tauri-apps/api/event'
import { invoke } from '@tauri-apps/api/tauri'
import pgk from '../../package.json'
import { ref } from 'vue';
import { useCounterStore } from '@/stores/counter'; 
import type { Picture } from '@/types';
import type { Duration } from '@tauri-apps/api/http';
const store = useCounterStore();

interface SpiderResult {
    pictures: Picture[],
    duration: Duration,
}

const data =ref()

let url = "http://dkleh8.xyz/pw/thread.php?fid=14";




function click() {
  invoke('spider_img', { url: url }).then((value) => {
    const res = value as SpiderResult
    console.log(res)
    store.setPictures(res.pictures as Picture[])
    // data.value = res
  })
}
</script>
<template>
  <div class="about">
    <h3>version {{ pgk.version }}</h3>
    <h4>author {{ pgk.author }}</h4>
    <h1>This is an about page</h1>
    <button @click="click">Click me</button>
    <div>{{ data }}</div>
  </div>
</template>

<style>

</style>
