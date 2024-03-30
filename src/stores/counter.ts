import { ref, computed } from 'vue'
import { defineStore } from 'pinia'

export const useCounterStore = defineStore('counter', () => {
  const count = ref(0)
  const sideShowFlag = ref(true)
  const doubleCount = computed(() => count.value * 2)
  const photos = {title:"Hot 魚子醬紫色連衣裙寫真 (94P)",srcs:[{"src":"https://pic2303e.xyz/i/2024/01/03/10qfemj.jpg","aspect_ratio":1.3333333333333333},{"src":"https://pic2303e.xyz/i/2024/01/03/10qhd0q.jpg","aspect_ratio":0.7547169811320755},{"src":"https://pic2303e.xyz/i/2024/01/03/10qhrgf.jpg","aspect_ratio":0.6666666666666666},{"src":"https://pic2303e.xyz/i/2024/01/03/10qg8ef.jpg","aspect_ratio":0.7547169811320755},{"src":"https://pic2303e.xyz/i/2024/01/03/10qfnm6.jpg","aspect_ratio":0.7843137254901961},{"src":"https://pic2303e.xyz/i/2024/01/03/10qfuyb.jpg","aspect_ratio":0.8163265306122449},{"src":"https://pic2303e.xyz/i/2024/01/03/10qjy7y.jpg","aspect_ratio":0.7339449541284404},{"src":"https://pic2303e.xyz/i/2024/01/03/10qgnf2.jpg","aspect_ratio":0.7339449541284404},{"src":"https://pic2303e.xyz/i/2024/01/03/10qhg17.jpg","aspect_ratio":0.684931506849315},{"src":"https://pic2303e.xyz/i/2024/01/03/10qjcs6.jpg","aspect_ratio":0.6666666666666666},{"src":"https://pic2303e.xyz/i/2024/01/03/10qn5wh.jpg","aspect_ratio":0.6666666666666666},{"src":"https://pic2303e.xyz/i/2024/01/03/10qo0rj.jpg","aspect_ratio":0.6666666666666666},{"src":"https://pic2303e.xyz/i/2024/01/03/10qgwr2.jpg","aspect_ratio":0.7339449541284404},{"src":"https://pic2303e.xyz/i/2024/01/03/10qm34b.jpg","aspect_ratio":0.6666666666666666},{"src":"https://pic2303e.xyz/i/2024/01/03/10qlh9o.jpg","aspect_ratio":0.7619047619047619},{"src":"https://pic2303e.xyz/i/2024/01/03/10qmgbo.jpg","aspect_ratio":0.6666666666666666},{"src":"https://pic2303e.xyz/i/2024/01/03/10qippa.jpg","aspect_ratio":0.7272727272727273},{"src":"https://pic2303e.xyz/i/2024/01/03/10qghm3.jpg","aspect_ratio":0.6956521739130435},{"src":"https://pic2303e.xyz/i/2024/01/03/10qkunv.jpg","aspect_ratio":0.6666666666666666},{"src":"https://pic2303e.xyz/i/2024/01/03/10qjl1s.jpg","aspect_ratio":0.6666666666666666},{"src":"https://pic2303e.xyz/i/2024/01/03/10qi9ic.jpg","aspect_ratio":0.6666666666666666},{"src":"https://pic2303e.xyz/i/2024/01/03/10qlmiq.jpg","aspect_ratio":0.6666666666666666},{"src":"https://pic2303e.xyz/i/2024/01/03/10qmtdd.jpg","aspect_ratio":0.6666666666666666},{"src":"https://pic2303e.xyz/i/2024/01/03/10qhadr.jpg","aspect_ratio":0.7142857142857143},{"src":"https://pic2303e.xyz/i/2024/01/03/10ql5s4.jpg","aspect_ratio":0.6666666666666666},{"src":"https://pic2303e.xyz/i/2024/01/03/10qiutq.jpg","aspect_ratio":0.6956521739130435},{"src":"https://pic2303e.xyz/i/2024/01/03/10qmxzo.jpg","aspect_ratio":0.6666666666666666},{"src":"https://pic2303e.xyz/i/2024/01/03/10qicvt.jpg","aspect_ratio":0.6666666666666666},{"src":"https://pic2303e.xyz/i/2024/01/03/10qg3mx.jpg","aspect_ratio":0.7079646017699115}]
} 
  function increment() {
    count.value++
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

  return {sideShowFlag, count, doubleCount,  mainWidth,mainHeight,photos,
    setMainWH,changeSideShowFlag, increment }
})
