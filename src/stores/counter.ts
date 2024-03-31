import { ref } from 'vue'
import { defineStore } from 'pinia'
import {data} from './data'
import type { Picture } from '@/types'

export const useCounterStore = defineStore('counter', () => {
  const cols = ref(4)
  const pictures = ref<Picture[]>(data)
  const sideShowFlag = ref(true)
  const photos = ref<Picture>(data[0])
  function changeCols(cb: (n: number) => number) {
    cols.value = cb(cols.value)
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

  return {sideShowFlag,pictures, cols,  mainWidth,mainHeight,photos,
    setMainWH,changeSideShowFlag,setPhotos,changeCols ,changePictures,updatePictures}
})
