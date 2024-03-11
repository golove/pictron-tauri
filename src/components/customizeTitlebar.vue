<template>
 <div data-tauri-drag-region class="titlebar">
    
      <i class="titlebar-button" id="titlebar-maximize" @click="$emit('showSide')">
        <IconSidebar />
      </i>
     
    </div>
</template>

<script setup lang="ts">

import IconSidebar from './icons/IconSidebar.vue'



const width = 150


</script>
<style>
.titlebar {
  width: v-bind(width +'px');
  height: 30px;
  user-select: none;
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  z-index: 1000;
}

.titlebar-button {
  user-select: none;
  position: absolute;
  align-items: center;
  width: auto;
  left: 80px;
  top: 5px;
  height: 20px;
  /* background-color: #333; */
}
</style>