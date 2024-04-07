<script setup lang="ts">
import { RouterLink, RouterView,useRoute } from 'vue-router'

import { invoke } from '@tauri-apps/api/tauri'
import { computed, ref,watch } from 'vue'
import IconSupport from './components/icons/IconSupport.vue'
import customizeTitlebar from './components/customizeTitlebar.vue'
import IconBack from './components/icons/IconBack.vue'
import IconForward from './components/icons/IconForward.vue'
import IconSearch from './components/icons/IconSearch.vue'
import IconApp from './components/icons/IconApp.vue'
import IconEcosystem from './components/icons/IconEcosystem.vue'
import IconWindow from './components/icons/IconWindow.vue'
import IconClose from './components/icons/IconClose.vue'
import IconDocumentation from './components/icons/IconDocumentation.vue'
import { storeToRefs } from 'pinia';
import {useCounterStore} from './stores/counter'
const {changeSideShowFlag,findPictures} = useCounterStore()
const store = useCounterStore()
const {mainWidth,sideShowFlag,maxCols,cols,pictureTitle } = storeToRefs(store)
const data = ref()
// const router = useRouter()
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
  icon: IconEcosystem
},
{
  to: '/collect',
  title: 'Collect',
  icon: IconSupport
},
{
  to: '/about',
  title: 'About',
  icon: IconDocumentation
}
]





const sideWidth = ref(200)
const marginToolbarButton = ref(0)

store.setMainWH(window.innerWidth - sideWidth.value,true)
store.setMainWH(window.innerHeight,false)
onresize = ()=>{
 store.setMainWH(window.innerWidth - sideWidth.value,true)
 store.setMainWH(window.innerHeight,false)
}

watch(()=>sideShowFlag.value,(n)=>{
  sideWidth.value = n?200:0
  store.setMainWH(window.innerWidth - sideWidth.value,true)
  marginToolbarButton.value = n ? 0 : 120
})

const sideMargin = computed(()=>sideShowFlag.value ? "4px 8px" : "0px")
const toolBarWidth = computed(()=>sideShowFlag.value ?"100%" : "calc(100% - 120px)" )

const searchText = ref('')
watch(()=>searchText.value,(n)=>{
  console.log(searchText.value)
  findPictures(searchText.value)
})
function clearSearchText(){
  searchText.value = ''
  
}

const pictureTitleFlag = ref(false)
const router = useRoute()
watch(()=>router.name,(n)=>{
  console.log(n)
  if(n === 'PictureView' || n === 'BigView'){
    pictureTitleFlag.value = true
  }else{
    pictureTitleFlag.value = false
  }
})



watch(()=>store.cols,(n)=>{
 console.log(n)
})

</script>

<template>

  <customizeTitlebar @showSide="changeSideShowFlag()" />
  <div :style="{width:sideWidth + 'px'}" class="side">

    
    <div class="searchBox">
          <i>
            <IconSearch />
          </i>
          <input v-model="searchText" type="text" name="search"
           id="search" 
           placeholder="search" />
          <i v-show="searchText" @click="clearSearchText"><IconClose /></i>
          
          
        </div>
    <router-link v-for="item in links" :key="item.title" :to="item.to">
      <i>
        <component :is="item.icon" />
      </i>
      <div class="title">{{ item.title }}</div>
    </router-link>
  </div>
  <main :style="{width:mainWidth + 'px',left:sideWidth + 'px'}"
   
  >
    <div data-tauri-drag-region class="toolbar">
      <div data-tauri-drag-region class="toolbarButton" :style="{left: marginToolbarButton + 'px'}">
        <div class="arrowButton">
          <i class="back" @click="$router.go(-1)">
            <IconBack />
          </i>
          <i class="forward" @click="$router.go(1)">
            <IconForward />
          </i>
        </div>
        <div v-show="!pictureTitleFlag" data-tauri-drag-region class="toolbarTitle">{{ $route.name }}</div>
        <div v-show="pictureTitleFlag" class="pictureTitle">{{ pictureTitle }}</div>
        <div class="scaleButton">
          <i class="magnify" @click="store.changeCols(()=>cols+1)">
            <IconApp /></i>
          <i class="shrink" @click="store.changeCols(()=>cols-1)">
          <IconWindow />
          <!-- <IconDocumentation /> -->
        </i>
        </div>
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
  margin-top: 30px;
  padding:v-bind(sideMargin) ;
  height: 100%;
  background-color: rgba(135, 135, 135, 0.1);
  overflow: hidden;
  transition: all 0.3s ease;
}

.searchBox {
  position: relative;
  width: 100%;
  /* top: 30px; */
  margin: 10px 0;
  align-items: center;
  /* border: 1px solid rgba(255, 255, 255, 0.3); */
  border-radius: 12px;
  display: flex;
  transition: all 0.3s ease;
  background-color: rgba(249, 249, 249, 0.508);
}
.searchBox i:nth-child(1){
  margin-left: 8px;
}

input#search {
  width: 90%;
  border: none;
  outline: none;
  height: 20px;
  background-color: rgba(0, 0, 0, 0);
  padding-left: 2px;
  font-size: 14px;
  color: var(--color-text);
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
  height: 25px;
  color: var(--color-text);
  fill: var(--color-text);
}

i:hover {

  background-color: var(--color-text-half2);
  border-radius: 6px;
  
}
i:hover svg {
  fill:#f88;
}
/* svg {
  user-select: none;
  fill: inherit;
  transition: all 0.3s ease;
} */


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
  background: rgba(92, 63, 74, 0.05);
}


.toolbar {
  user-select: none;
  height: 30px;
  width: inherit;
  /* background: var(--color-background-bar); */
 
  background-color: var(--color-background-soft);
  position: fixed;
  top: 0;
}

.toolbarButton {
  user-select: none;
  position: absolute;
  margin-left: 4px;
  top: 2.5px;
  width: v-bind(toolBarWidth);
  height: 100%;
  transition: all 0.3s ease;
}

.arrowButton {
  user-select: none;
  position: absolute;
  top: 0;
  align-items: center;
  display: flex;
  color: var(--color-text);
  
}

.scaleButton{
  user-select: none;
  position: absolute;
  top: 0;
  right: 20px;
  align-items: center;
  display: flex;
  color: var(--color-text);
}
.toolbarTitle{
  user-select: none;
  position: absolute;
  left: 80px;
}
.pictureTitle{
  position: absolute;
  left: 50%;
  height: inherit;
  transform: translateX(-50%);
  overflow: hidden;
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
  background-color: var(--color-text-half);
  border-radius: 3px;
  cursor: pointer;
}

/* .content::-webkit-scrollbar-track {
  background-color: var(--color-background);
} */
</style>
