import { ref, computed } from 'vue'
import { defineStore } from 'pinia'

export const useCounterStore = defineStore('counter', () => {
  const count = ref(0)
  const sideShowFlag = ref(true)
  const doubleCount = computed(() => count.value * 2)
  function increment() {
    count.value++
  }
  function changeSideShowFlag() {
    sideShowFlag.value = !sideShowFlag.value
  }
  const mainWidth = ref(0)
  function setMainWidth(n:number){
    mainWidth.value = n
  }

  return {sideShowFlag, count, doubleCount,  mainWidth,
    setMainWidth,changeSideShowFlag, increment }
})
