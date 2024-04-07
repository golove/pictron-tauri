<script setup lang="ts">
import { onMounted, reactive, ref, watch, watchEffect,computed } from 'vue'
import { images } from '@/utils'
import { useRoute, useRouter } from 'vue-router'
import ChevronRight from '@/components/icons/ChevronRight.vue'
import ChevronLeft from '@/components/icons/ChevronLeft.vue'
import { storeToRefs } from 'pinia';
import { useCounterStore } from '@/stores/counter'
// 控制全屏显示
const fullScreenFlag = ref(false)
function toFullScreen() {
    fullScreenFlag.value = !fullScreenFlag.value
}
const store = useCounterStore()
const { mainWidth,mainHeight, sideShowFlag } = storeToRefs(store)

const props = defineProps<{
    photos: {
        src: string,
        aspect_ratio: number
    }[]
}>()
const route = useRoute()
const router = useRouter()
// 如果图片数据为空，则返回上一页
if (!props.photos) {
    router.go(-1)
}
const photos = images<{ src: string, aspect_ratio: number }>(props.photos)

const bigContainer = ref<HTMLElement>()
const smallPhoto = ref<HTMLElement>()
const bigPhoto = ref<HTMLElement>()
const smallPhotoHeight = computed(()=>fullScreenFlag.value ? 0 : 65)
const currentIndex = ref(1)
const widthSet = reactive({
    bigWidth: new Array(photos.length).fill(0),
    smallWidth: new Array(props.photos.length).fill(0),
})
const oldWidth = {
    bigWidth: 0,
    smallWidth: 0,
}

const containerHeight = computed(() => mainHeight.value - smallPhotoHeight.value )

const showTypeFlag = ref(false)

function setWidth(e: { src: string, aspect_ratio: number }, n: number, name: 'bigWidth' | 'smallWidth') {
    if (!bigPhoto.value && !smallPhoto.value) return
    const hei = name === 'bigWidth' ? containerHeight.value : smallPhotoHeight.value
    const width = name === 'bigWidth' ? (showTypeFlag.value ? hei * e.aspect_ratio : mainWidth.value) : hei * e.aspect_ratio + 2
    widthSet[name][n] = {
        width: width + 'px',
        left: oldWidth[name] + 'px'
    }
    oldWidth[name] += width
}



// 监听路由参数的变化，更新currentIndex
// watchEffect(() => {
//     // currentIndex.value = +route.params.index + 1
//     if (bigContainer.value && smallPhoto.value && bigPhoto.value) {
//         smallPhoto.value.style.height = `${smallPhotoHeight.value}px`
//         bigContainer.value.style.height = `${containerHeight.value}px`
//         bigPhoto.value.style.height = `${containerHeight.value}px`
//     }
//     init(photos, 'bigWidth')
//     init(props.photos, 'smallWidth')
    
// })

watch(()=>route.params.index, (n) => {
    currentIndex.value = (+n + 1)
})


onMounted(() => {
    if (bigContainer.value && smallPhoto.value && bigPhoto.value) {
        smallPhoto.value.style.height = `${smallPhotoHeight.value}px`
        bigContainer.value.style.height = `${containerHeight.value}px`
        bigPhoto.value.style.height = `${containerHeight.value}px`
    }
    init(photos, 'bigWidth')
    init(props.photos, 'smallWidth')
})

function init(arr: { src: string, aspect_ratio: number }[], name: 'bigWidth' | 'smallWidth') {
    oldWidth[name] = 0
    for (let index = 0; index < arr.length; index++) {
        const e = arr[index];
        const divRef = name === 'bigWidth' ? bigPhoto.value : smallPhoto.value
        if (!divRef) return
        setWidth(e, index, name)
        if (index === arr.length - 1) {
            divRef.style.width = (+widthSet[name][arr.length - 1].left.split('px')[0] + +widthSet[name][arr.length - 1].width.split('px')[0]) + 'px'
            if (name === 'smallWidth') {
                sliderImgclick(currentIndex.value - 1)
                moveTo(currentIndex.value, false)
                console.log(widthSet)
            }
        }

    }

}



function smallPhotoClick(cb: () => void) {
    cb()
    // 处理图片的无缝循环
    if (currentIndex.value > photos.length - 2) {
        currentIndex.value = 0
        moveTo(currentIndex.value, false)
        currentIndex.value++
    } else if (currentIndex.value < 1) {
        currentIndex.value = photos.length - 1

        moveTo(currentIndex.value, false)
        currentIndex.value--
    }
    sliderImgclick(currentIndex.value - 1)
    moveTo(currentIndex.value)

}

// 移动到指定索引的图片
function moveTo(n: number, flag: boolean = true) {
    if (!bigPhoto.value || !smallPhoto.value || !bigContainer.value) return
    bigPhoto.value.style.transition = flag ? 'transform 0.55s ease' : 'none'
    bigPhoto.value.style.transform = `translateX(-${widthSet['bigWidth'][n].left})`

}



const foucestyle = ref({
    width: '0px',
    left: '0px',
})

function sliderImgclick(n: number, flag: boolean = true) {
    foucestyle.value = widthSet['smallWidth'][n]
    if (smallPhoto.value && bigContainer.value) {
        const w1 = +widthSet['smallWidth'][n].left.split('p')[0]
        const w2 = (mainWidth.value) * 0.76       //从100%开始移动 -(+widthSet['smallWidth'][n].width.split('p')[0])
        const dis = w1 - w2 > 0 ? w1 - w2 : 0
        smallPhoto.value.style.transition = flag ? 'transform 0.55s ease' : 'none'
        smallPhoto.value.style.transform = `translateX(-${dis}px)`

    }
}

const opacityArrowLeft = ref(0)
const opacityArrowRight = ref(0)


// 键盘事件处理
interface KeySet {
    ArrowLeft: () => void
    ArrowRight: () => void
    Space: () => void
    ArrowDown: () => void
    ArrowUp: () => void
}
const keySet: KeySet = {
    ArrowLeft: () => smallPhotoClick(() => {
        opacityArrowLeft.value = 1
        currentIndex.value--
    }),
    ArrowRight: () => smallPhotoClick(() => {
        opacityArrowRight.value = 1
        currentIndex.value++
    }),
    Space: () => smallPhotoClick(() => {
        opacityArrowRight.value = 1
        currentIndex.value++
    }),
    ArrowDown: toFullScreen,
    ArrowUp: toFullScreen
}
// 添加键盘事件监听
document.addEventListener('keydown', (e: KeyboardEvent) => {
    const handler = keySet[e.code as keyof KeySet]
    if (handler) {
        handler()
    }
})

function setTimer(name: string) {
    const timer = setTimeout(() => {
        name === 'ArrowLeft' ? opacityArrowLeft.value = 0 : opacityArrowRight.value = 0
        clearTimeout(timer)
    }, 1000);
}
const keyupSet: KeySet = {
    ArrowLeft: () => smallPhotoClick(() => setTimer('ArrowLeft')),
    ArrowRight: () => smallPhotoClick(() => setTimer('ArrowRight')),
    Space: () => smallPhotoClick(() => setTimer('ArrowRight')),
    ArrowDown: () => { },
    ArrowUp: () => { }
}
document.addEventListener('keyup', (e: KeyboardEvent) => {
    const handler = keyupSet[e.code as keyof KeySet]
    if (handler) {
        handler()
    }
})




// watch(()=>fullScreenFlag.value, (n) => {
//     smallPhotoHeight.value = n ? 0 : 65
// })


</script>
<template>

    <div class="bigview">
        <div ref="bigContainer" class="bigContainer">
            <div class="arrLeft" @click="smallPhotoClick(() => currentIndex--)">
                <ChevronLeft />
            </div>
            <div class="arrRight" @click="smallPhotoClick(() => currentIndex++)">
                <ChevronRight />
            </div>

            <div ref="bigPhoto" class="bigPhoto">
                <div class="img" v-for="(e, i) in photos" :style="widthSet['bigWidth'][i]" :key="e.src">
                    <img :src="e.src" alt="">
                </div>
            </div>

        </div>
        <div ref="smallPhoto" class="smallPhoto">
            <div className='fouceStyle' :style="foucestyle"></div>
            <div class="img" v-for="(e, i) in props.photos" @click="smallPhotoClick(() => currentIndex = i + 1)"
                :style="widthSet['smallWidth'][i]" :key="e.src">
                <img :src="e.src" alt="">
            </div>
        </div>

    </div>
</template>
<style scoped>
.bigview {
  
    width: 100%;
    height: inherit;
    overflow: hidden;
}

.bigContainer {
    position: relative;
    width: 100%;
    transition: all 0.3s ease;
}

.bigPhoto {
    height: 100%;
    margin-top: 0.5%;
    overflow: hidden;

}

.bigPhoto .img,
.smallPhoto .img {
    height: 100%;
    width: 100%;
    position: absolute;

}

.bigPhoto .img img {
    height: 100%;
    width: auto;
    margin: 0 50%;
    transform: translateX(-50%);
    border-width: 3px;
    border-style: solid;
    /* border-image: linear-gradient(to right, #ff6b6b, #ffb677, #fdff94, #d4fc79, #96e6a1); */
    border-image-slice: 1;

}

.smallPhoto .img img {
    height: 100%;
    width: auto;

}

.arrRight,
.arrLeft {
    position: absolute;
    height: 100%;
    width: 120px;
    fill: rgba(255, 255, 255, 0.656);
    transition: opacity .3s ease-in-out;
    z-index: 99999;
}

.arrRight {
    opacity: v-bind(opacityArrowRight);
}

.arrLeft {
    opacity: v-bind(opacityArrowLeft);
}

.arrRight:hover,
.arrLeft:hover {
    opacity: 1;
    fill: #ff6b6b;
}

.arrRight svg,
.arrLeft svg {
    position: absolute;
    top: 50%;
    transform: translateY(-50%);
}

.arrRight {
    right: 0;
}

.smallPhoto {
    position: relative;
    margin-top: 0.5%;
  
    overflow: hidden;
    transition: all 0.3s ease;
}

.fouceStyle {
    position: absolute;
    height: 100%;
    border-width: 3px;
    border-style: solid;
    /* border-image: linear-gradient(to right, #ff6b6b, #ffb677, #fdff94, #d4fc79, #96e6a1); */
    border-image-slice: 1;
    z-index: 999;
    transition: all .3s ease-in-out;
    /* animation: hueRotate 5s infinite linear; */
}

/* @keyframes hueRotate {
    100% {
        filter: hue-rotate(360deg)
    } 
} */
</style>