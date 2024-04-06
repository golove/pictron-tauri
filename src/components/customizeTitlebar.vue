<template>
 <div data-tauri-drag-region class="titlebar">
    
      <i class="titlebar-button" id="titlebar-maximize" @click="$emit('showSide')">
        <IconSidebar />
      </i>
     
    </div>
</template>

<script setup lang="ts">

import { computed } from 'vue';
import IconSidebar from './icons/IconSidebar.vue'
import { useCounterStore } from '@/stores/counter';
const store = useCounterStore()

const width = computed(()=>store.sideShowFlag? 200 : 100)




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
  background-color: rgba(135, 135, 135, 0.1);
}

.titlebar-button {
  user-select: none;
  position: absolute;
  align-items: center;
  width: auto;
  left: 80px;
  top: 2.5px;
  height: 25px;
  /* background-color: #333; */
}
</style>