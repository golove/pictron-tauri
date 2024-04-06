<script setup lang="ts">
import { invoke } from '@tauri-apps/api/tauri'
import pgk from '../../package.json'
import { ref } from 'vue';

const data =ref()



function click() {
  invoke('spider').then((res) => {
    data.value = res
  })
}
</script>
<template>
  <div class="about">
    <h3>version {{ pgk.version }}</h3>
    <h1>This is an about page</h1>
    <button @click="click">Click me</button>
    <div>{{ data }}</div>
  </div>
</template>

<style>

</style>
