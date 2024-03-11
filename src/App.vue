<script setup lang="ts">
import { RouterLink, RouterView } from 'vue-router'
import { invoke } from '@tauri-apps/api/tauri'
import { ref,watch } from 'vue'
import HomeIcon from './components/icons/IconDocumentation.vue'
import IconSupport from './components/icons/IconSupport.vue'
import customizeTitlebar from './components/customizeTitlebar.vue'
import IconBack from './components/icons/IconBack.vue'
import IconForward from './components/icons/IconForward.vue'
import IconSearch from './components/icons/IconSearch.vue'
import IconClose from './components/icons/IconClose.vue'
import {useCounterStore} from './stores/counter'
const {changeSideShowFlag} = useCounterStore()
const store = useCounterStore()

const data = ref()
invoke('showName', { name: 'World' })
  // `invoke` 返回异步函数
  .then((response) => console.log(response))

function click() {
  invoke('spider').then((res) => {
    data.value = res
  })
}

const links = [{
  to: '/',
  title: 'Home',
  icon: HomeIcon
},
{
  to: '/about',
  title: 'About',
  icon: IconSupport
}
]




const sideWidth = ref(150)
const marginToolbarButton = ref(0)

store.setMainWidth(window.innerWidth - sideWidth.value)
onresize = ()=>{
 store.setMainWidth(window.innerWidth - sideWidth.value)
 console.log('window',store.mainWidth)
}

watch(()=>store.sideShowFlag,(n)=>{
  console.log(n)
  sideWidth.value = n?150:0
  store.setMainWidth(window.innerWidth - sideWidth.value)
  marginToolbarButton.value = n ? 0 : 120
})

  


const searchText = ref('')
function clearSearchText(){
  searchText.value = ''
}
</script>

<template>

  <customizeTitlebar @showSide="changeSideShowFlag()" />
  <div :style="{width:sideWidth + 'px'}" class="side">

    <div class="avatar">golove</div>
    <router-link v-for="item in links" :key="item.title" :to="item.to">
      <i>
        <component :is="item.icon" />
      </i>
      <div class="title">{{ item.title }}</div>
    </router-link>
  </div>
  <main :style="{width:store.mainWidth + 'px',left:sideWidth + 'px'}">
    <div data-tauri-drag-region class="toolbar">
      <div data-tauri-drag-region class="toolbarButton" :style="{left: marginToolbarButton + 'px'}">
        <div class="arrowButton">
          <i class="back">
            <IconBack />
          </i>
          <i class="forward">
            <IconForward />
          </i>
        </div>

      </div>
      <div class="searchBox">
          <i>
            <IconSearch />
          </i>
          <input v-model="searchText" type="text" name="search" id="search" placeholder="search" />
          <i v-show="searchText" @click="clearSearchText"><IconClose /></i>
          
        </div>
    </div>
    <div class="content">
      <RouterView />
    </div>
    
  </main>
</template>

<style>
.side {
  position: relative;
  user-select: none;
  height: 100%;
  background-color: rgba(225, 225, 225, 0.1);
  overflow: hidden;
  transition: all 0.3s ease;
  border-radius: 12px 0 0 0;
}

.avatar {
  user-select: none;
  border-radius: 12px 0 0 0;
  position: relative;
  width: inherit;
  margin-top: 30px;
  height: 50px;
}

a {
  user-select: none;
  width: 100%;
  font-weight: bold;
  display: inline-flex;
  text-align: center;
  color: var(--color-text);

}

a:hover {
  background-color: var(--color-background);
  color: var(--color-svg);
  fill: var(--color-svg);
}

i {
  user-select: none;
  display: flex;
  place-items: center;
  place-content: center;
  width: 32px;
  height: 32px;
  color: var(--color-text);
}

svg {
  user-select: none;
  fill: var(--color-text);
  transition: all 0.3s ease;
}


.title {
  user-select: none;
  flex: 1;
  margin-left: 0rem;
}

main {
  height: 100%;
  top: 30px;
  position: absolute;
  min-height: 100vh;
  transition: all 0.3s ease;
}


.toolbar {
  user-select: none;
  height: 30px;
  width: inherit;
  background: rgba(92, 63, 74, 0.31);
  /* background-color: var(--color-background-soft); */
  position: fixed;
  top: 0;
}

.toolbarButton {
  user-select: none;
  position: absolute;
  top: 0;
  width: inherit;
  height: 70%;
  transition: all 0.3s ease;
}

.arrowButton {
  user-select: none;
  position: absolute;
  top: 0;
  align-items: center;
  display: flex;
  color: var(--color-text);
  ;
}


.searchBox {
  position: absolute;
  width: 220px;
  top: 0;
  right: 10px;
  height: 20px;
  top: 5px;
  align-items: center;
  border: 1px solid rgba(255, 255, 255, 0.3);
  border-radius: 12px;
  display: flex;
  transition: all 0.3s ease;
}

input#search {
  border: none;
  outline: none;
  height: 20px;
  background-color: rgba(0, 0, 0, 0);
  padding-left: 2px;
  font-size: 14px;
  color: var(--color-text);
}

.content{
  position: absolute;
  width: 100%;
  height: calc(100vh - 30px);
  overflow-y: scroll;
}
.content::-webkit-scrollbar {
  width: 6px;
}
.content::-webkit-scrollbar-thumb {
  background-color: rgba(151, 151, 151, 0.3);
  border-radius: 3px;
  cursor: pointer;
}

.content::-webkit-scrollbar-track {
  background-color: rgba(41, 41, 41, 0.3);
}
</style>
