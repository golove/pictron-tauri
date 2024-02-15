<script setup lang="ts">
import { RouterLink, RouterView } from 'vue-router'

import { invoke } from '@tauri-apps/api/tauri'
import { ref } from 'vue'
const data = ref()
invoke('showName', { name: 'World' })
  // `invoke` 返回异步函数
  .then((response) => console.log(response))

function click() {
  invoke('spider').then((res) => {
    data.value = res
  })
}
</script>

<template>
  <header>
    <nav>
      <RouterLink to="/">Home</RouterLink>
      <RouterLink to="/about">About</RouterLink>
      <button @click="click">click</button>
    </nav>
  </header>
{{ data }}
  <RouterView />
</template>

<style scoped>
header {
  line-height: 1.5;
  height: 60px;
  width: 100vw;
}

nav {
  width: 100%;
  font-size: 12px;
  text-align: center;
  margin-top: 2rem;
}

nav a {
  display: inline-block;
  padding: 0 1rem;
  border-left: 1px solid var(--color-border);
}
</style>
