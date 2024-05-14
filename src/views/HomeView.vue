<script setup lang="ts">
import { computed, ref } from 'vue'
import { listen } from '@tauri-apps/api/event'
import masonry from '../components/gMasonry.vue'
import gCard from '../components/gCard.vue'
import ToolBar from '@/components/ToolKit.vue'
import rate from '@/components/gRate.vue'
import { useRouter } from 'vue-router'
import { storeToRefs } from 'pinia';
import { useCounterStore } from '@/stores/counter';
import { invoke } from '@tauri-apps/api'
import { appDataDir } from '@tauri-apps/api/path';
import { ask, open } from '@tauri-apps/api/dialog';
// import { sendNotification } from '@tauri-apps/api/notification';


const store = useCounterStore()
const { cols, pictures, filterData } = storeToRefs(store)
const imgs = computed(() => filterData.value.length ? filterData.value : pictures.value)
const router = useRouter()
function click(title: string, n: number) {
  store.setPhotos(n)
  store.setPictureTitle(title)
  router.push(`/PictureView/${title}`)
}

function updateModelValue(obj: { id: number, value: number }) {
  store.updatePictures({ id: obj.id, fieldsToUpdate: { star: obj.value } })
}

interface DownloadEventName {
  event: string
  id: number,
  count: number
}

const eventListenerList = ref<DownloadEventName[]>([])



function update(obj: { type: 'collect' | 'download' | 'deleted', id: number, value: boolean }) {
  console.log(obj.id, obj.type, obj.value)
  store.updatePictures({ id: obj.id, fieldsToUpdate: { [obj.type]: obj.value } })
  if (obj.type === 'deleted') {
    store.changePictures(obj.id)
  }
  if (obj.type === 'download' && obj.value) {
    let img = imgs.value.find(i => i.id === obj.id)
    if (img) {
      invoke("get_folder_path").then(async (res) => {
        console.log(res)
        if (res) {
          eventListenerList.value.push({ event: "download-success" + img.id, id: img.id, count: 0 })
          invoke('download_img', { srcs: img.srcs, title: img.title, id: img.id }).then(res => {
            console.log(res)
          })

          downloadSuccess(img.id)
        } else {
          const yes = await ask("选择保存文件夹", '默认保存在Download文件夹下，是否更改？');
          // const yes2 = await ask('This action cannot be reverted. Are you sure?', { title: 'Tauri', type: 'warning' });


          if (yes) {
            const selected = await open({
              directory: true,
              multiple: true,
              defaultPath: await appDataDir(),
            }
            )
            let path = selected as string[];
            if (path.length === 0) return;
            invoke("insert_config",
              { path: path[0] }
            ).then((res) => {
              console.log(res);
              if (res === "ok") {
                eventListenerList.value.push({ event: "download-success" + img.id, id: img.id, count: 0 })
                invoke('download_img', { srcs: img.srcs, title: img.title, id: img.id }).then(res => {
                  console.log(res)
                })
              }
            });
          } else {
            eventListenerList.value.push({ event: "download-success" + img.id, id: img.id, count: 0 })
            invoke('download_img', { srcs: img.srcs, title: img.title, id: img.id }).then(res => {
              console.log(res)
            })
          }
        }
      })
    }
  }
}





// const downloadCount = ref(0)
interface DownloadSuccessEvent {
  msg: number
}



function downloadSuccess(id: number) {
  eventListenerList.value.map(item => {
    if (item.id === id) {
      listen(item.event, (event) => {
        if (event && event.payload) {
          let res = event.payload as DownloadSuccessEvent;
          item.count = res.msg
        }
      })
    }

  })

}



let dcount = (id: number) => {
  if (eventListenerList.value.length === 0) return 0
  let items = eventListenerList.value.filter(item => item.id === id)
  if (items.length === 0) return 0
  console.log(items[0].count)
  return items[0].count

}




</script>

<template>

  <masonry :cols="cols" :gap="10">
    <gCard v-for="(img, i) in imgs" :aspectRatio="img.srcs[0].aspect_ratio" :key="img.id" :src="img.srcs[0].src"
      :title="img.title" :width="230" absolute @click="click(img.title, i)">
      <template #header>{{ img.title }} </template>
      <template #toolkit>
        <rate :id="img.id" :size="16" :value="img.star" @update="updateModelValue" />
        <ToolBar :id="img.id" :like="img.collect" :download="img.download" :deleted="img.deleted" @update="update"
          :length="img.srcs.length" :downloadCount="dcount(img.id)" />
      </template>
    </gCard>
  </masonry>

</template>
<style></style>
