<script setup lang="ts">
import { emit, listen ,type Event} from '@tauri-apps/api/event'
import { invoke } from '@tauri-apps/api/tauri'
import pgk from '../../package.json'
import { ref } from 'vue';
import { useCounterStore } from '@/stores/counter'; 
import type { Picture } from '@/types';
const store = useCounterStore();


const data =ref()

let url = "http://dkleh8.xyz/pw/thread.php?fid=14";

listen('event-name',(e:Event<any>)=>{
  console.log(e.payload)
})


function click() {
  invoke('init_process').then((res) => {
    console.log(res)
    store.setPictures(res as Picture[])
    data.value = res
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
