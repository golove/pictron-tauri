<script setup lang="ts">

import { invoke } from '@tauri-apps/api/tauri'
import pgk from '../../package.json'
import { ref } from 'vue';
import { useCounterStore } from '@/stores/counter'; 
import type { Picture ,SpiderResult} from '@/types';

import { sendNotification } from '@tauri-apps/api/notification';

const store = useCounterStore();



const data =ref()

let url = "http://dkleh8.xyz/pw/thread.php?fid=14";




function click() {
  invoke('spider_img', { url: url }).then((value) => {
    const res = value as SpiderResult
    // console.log(res)
    store.setPictures(res.pictures)
    sendNotification({ title: 'Pictron', body: '从网络成功爬取图片' });
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
