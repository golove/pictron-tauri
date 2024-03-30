import { onUnmounted, ref } from 'vue'

export function findMax(array: number[]) {
  let index = 0
  let max = array[0]
  for (let i = 0; i < array.length; i++) {
    if (array[i] > max) {
      index = i
      max = array[i]
    }
  }
  return index
}


export const findMin = (array: number[]) => {
  let index = 0
  let min = array[0]
  for (let i = 0; i < array.length; i++) {
    if (array[i] < min) {
      index = i
      min = array[i]
    }
  }
  return index
}




// 防抖
// export const debounce = (fn: Function, delay: number) => {
//   let timer: any = null
//   return function () {
//     if (timer) {
//       clearTimeout(timer)
//     }
//     timer = setTimeout(() => {
//       // eslint-disable-next-line prefer-rest-params
//       fn.apply(this, arguments)
//     }, delay)
//   }
  
// }

//
export function useDefer(maxCount: number = 23) {
  const frameCount = ref(1)
  let rafID: number
  function updateFrameCount() {
    rafID = requestAnimationFrame(() => {
      frameCount.value++
      updateFrameCount()
      if (frameCount.value >= maxCount) {
        return
      }
    })
  }
  updateFrameCount()
  onUnmounted(() => {
    cancelAnimationFrame(rafID)
  })
  return function (n: number) {
    return frameCount.value >= n
  }
}


  // 为照片数组添加首尾元素，用于循环播放
export function images<T>(srcs: T[]) {
    const first = srcs[0]
    const last = srcs[srcs.length - 1]
    return [last, ...srcs, first]
  }



