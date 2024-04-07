import { computed, ref,watch } from 'vue'
import { defineStore } from 'pinia'
import {data} from './data'
import type { Picture } from '@/types'

export const useCounterStore = defineStore('counter', () => {
  const cols = ref(3)
  const pictures = ref<Picture[]>(data)
  const filterData = ref<Picture[]>([])
  const sideShowFlag = ref(true)
  const maxCols = computed(() => Math.floor(mainWidth.value / 200))
  const minCols = computed(() => mainWidth.value<400?1:2)
  const pictureTitle = ref('')
  const photos = ref<Picture>(data[0])
  const collect = computed<Picture[]>(()=>pictures.value.filter(e=>e.collect))
  function changeCols(cb: (n: number) => number) {
    const n = cb(cols.value)
    if (n >= minCols.value && n <= maxCols.value) {
      cols.value = n
    }else if(n < minCols.value){
      cols.value = minCols.value
    }else{
      cols.value = maxCols.value
    }
  }
  function changeSideShowFlag() {
    sideShowFlag.value = !sideShowFlag.value
  }
  const mainWidth = ref(0)
  const mainHeight = ref(0)
  function setMainWH(n:number,falg:boolean){
    if(falg){
      mainWidth.value = n
    }else{
      mainHeight.value = n - 45
    }
  }
  function setPhotos(n:number){
    photos.value = pictures.value[n]
  }

  // 查找pictures中title包含keyword的图片并返回
  function findPictures(keyword: string) {
    console.log(keyword)
    filterData.value = pictures.value.filter((e: Picture) => e.title.includes(keyword))
  }

  watch(mainWidth,()=>{
    cols.value = Math.floor(mainWidth.value / 200)
  })

  function changePictures(_id: string) {
    pictures.value = pictures.value.filter((e: Picture) => e._id !== _id)
  }

  function updatePictures(obj: { _id: string; fieldsToUpdate: Partial<Picture> }) {
    pictures.value = pictures.value.map((e: Picture) => {
      if (e._id === obj._id) {
        e = { ...e, ...obj.fieldsToUpdate }
        // fetchUpdate({ _id: obj._id, ...obj.fieldsToUpdate })
      }
      return e
    })
  }

  function setPictureTitle(title: string) {
    pictureTitle.value = title
  }

  return {sideShowFlag,pictures, cols,  mainWidth,mainHeight,photos,collect,filterData,maxCols,pictureTitle,
    setMainWH,changeSideShowFlag,setPhotos,changeCols ,findPictures,setPictureTitle,
    changePictures,updatePictures}
})
