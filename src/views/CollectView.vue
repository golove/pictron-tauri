<script setup lang="ts">
import masonry from '../components/gMasonry.vue'
import gCard from '../components/gCard.vue'
import { useRouter } from 'vue-router'
import { storeToRefs } from 'pinia';
import { useCounterStore } from "../stores/counter";
const store = useCounterStore()
const { cols, collect } = storeToRefs(store)

const router = useRouter()
function click(title: string, n: number) {
    router.push(`/bigView/${title}/${n}`)
}

function updateModelValue(obj:{id:string,value:number}) {
    store.updatePictures({_id:obj.id,fieldsToUpdate:{star:obj.value}})
}

function update(obj:{type:'collect'|'delete',id:string,value:boolean}){
  store.updatePictures({_id:obj.id,fieldsToUpdate:{[obj.type]:obj.value}})
  if(obj.type==='delete'){
    store.changePictures(obj.id)
  }
}

</script>

<template>



    <masonry :cols="cols" :gap="10">
        <gCard v-for="(img,i) in collect" :aspectRatio="img.srcs[0].aspect_ratio" :key="img._id"
         :src="img.srcs[0].src" :title="img.title" :width="230" absolute @click="click(img.title,i)" >
         <template #header>{{ img.title }} </template>
        <template #toolkit>
          <rate :id="img._id" :size="16" :value="img.star" @update="updateModelValue" />
          <ToolBar :id="img._id" :like="img.collect" :download="img.download" 
           @update="update" :length="img.srcs.length" />  
        </template>
        </gCard>
    </masonry>

</template>

