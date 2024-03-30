<script setup lang="ts">
import { onMounted, reactive, ref, watch, watchEffect, computed } from 'vue'
import { images } from '@/utils'
import ChevronRight from '@/components/icons/ChevronRight.vue'
import ChevronLeft from '@/components/icons/ChevronLeft.vue'
import { storeToRefs } from 'pinia';
import { useCounterStore } from '@/stores/counter'
const smallPhotoContainer = ref<HTMLElement>()
const smallPhoto = ref<HTMLElement>()
const bigContainer = ref<HTMLElement>()
const bigPhoto = ref<HTMLElement>()
const currentIndex = ref(1)
const store = useCounterStore()
const { mainWidth, mainHeight, sideShowFlag } = storeToRefs(store)
const props = defineProps<{
    photos: {
        src: string,
        aspect_ratio: number
    }[]
}>()

// 控制全屏显示
const fullScreenFlag = ref(false)
function toFullScreen() {
    fullScreenFlag.value = !fullScreenFlag.value
}
const smallPhotoHeight = computed(() => fullScreenFlag.value ? 0 : 85)
const containerHeight = computed(() => mainHeight.value - smallPhotoHeight.value)

const photos = images<{ src: string, aspect_ratio: number }>(props.photos)
const widthSet = reactive({
    bigWidth: new Array(photos.length).fill(0),
    smallWidth: new Array(props.photos.length).fill(0),
})

const oldWidth = {
    bigWidth: 0,
    smallWidth: 0,
}

function setWidth(e: { src: string, aspect_ratio: number }, n: number, name: 'bigWidth' | 'smallWidth') {
    if (!bigPhoto.value && !smallPhoto.value) return
    const hei = name === 'bigWidth' ? containerHeight.value : smallPhotoHeight.value - 10
    const width = name === 'bigWidth' ? mainWidth.value : hei * e.aspect_ratio + 2
    widthSet[name][n] = {
        width: width + 'px',
        left: oldWidth[name] + 'px'
    }
    oldWidth[name] += width
}

function smallPhotoClick(cb: () => void) {
    cb()
    // 处理图片的无缝循环
    if (currentIndex.value > photos.length - 2) {
        currentIndex.value = 0
        moveTo(currentIndex.value, false)
        currentIndex.value++
        bigPhoto.value?.getClientRects() //强制渲染
    } else if (currentIndex.value < 1) {
        currentIndex.value = photos.length - 1
        moveTo(currentIndex.value, false)
        currentIndex.value--
        bigPhoto.value?.getClientRects() //强制渲染
    }
    sliderImgclick(currentIndex.value - 1)
    moveTo(currentIndex.value, true)

}

// 移动到指定索引的图片
function moveTo(n: number, flag: boolean = true) {
    console.log(flag)
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
    // console.log(n)
    if(!smallPhotoContainer.value || !smallPhoto.value) return
    
    const w1 = widthSet['smallWidth'][n].left.split('px')[0]
    const w2 = smallPhotoContainer.value.clientWidth
    const w3 = smallPhotoContainer.value.scrollWidth
    const thumbWidth =  (w2 / w3) * w2
    const thumbLeft = (w1 / w2) * (w2 - thumbWidth)
    console.log(thumbLeft)
    smallPhotoContainer.value?.scrollTo({
        left:thumbLeft ,
        behavior: 'auto'
    })
 
}


// 监听屏幕大小变化
watch(()=>[mainWidth.value, mainHeight.value,fullScreenFlag.value],()=>{
    if (bigContainer.value && smallPhoto.value && bigPhoto.value) {
        bigContainer.value.style.height = `${containerHeight.value}px`
        bigContainer.value.style.width = `${mainWidth.value}px`
    }
    init(photos, 'bigWidth')
    init(props.photos, 'smallWidth')
    moveTo(currentIndex.value,false)
})

onMounted(() => {
    if (bigContainer.value && smallPhoto.value && bigPhoto.value) {
        bigContainer.value.style.height = `${containerHeight.value}px`
        bigContainer.value.style.width = `${mainWidth.value}px`
    }
    init(photos, 'bigWidth')
    init(props.photos, 'smallWidth')

    sliderImgclick(currentIndex.value - 1)
    moveTo(currentIndex.value, false)

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
        }
    }

}

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
        // opacityArrowLeft.value = 1
        currentIndex.value--
    }),
    ArrowRight: () => smallPhotoClick(() => {
        // opacityArrowRight.value = 1
        currentIndex.value++
    }),
    Space: () => smallPhotoClick(() => {
        // opacityArrowRight.value = 1
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

</script>
<template>
    <div class="bigView">
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
        <div ref="smallPhotoContainer" class="smallPhotoContainer" :style="{ height: smallPhotoHeight + 'px' }">
            <div ref="smallPhoto" class="smallPhoto" :style="{ height: smallPhotoHeight - 10 + 'px' }">
                <div className='fouceStyle' :style="foucestyle"></div>
                <div class="img" v-for="(e, i) in props.photos" @click="smallPhotoClick(() => currentIndex = i + 1)"
                    :style="widthSet['smallWidth'][i]" :key="e.src">
                    <img :src="e.src" alt="">
                </div>
            </div>
        </div>
    </div>
</template>
<style>
.bigView {
    position: relative;
    width: 100%;
    height: 100%;
    overflow: hidden;
}

.bigContainer {
    position: relative;
}

.bigPhoto {
    position: absolute;
    height: 100%;
    overflow: hidden;
    margin-top: 0.5%;
}

.smallPhotoContainer {

    width: 100%;
    position: absolute;
    bottom: 0;
    overflow-y: hidden;
    overflow-x: scroll;

}


.smallPhotoContainer::-webkit-scrollbar {
    height: 6px;
}

.smallPhotoContainer::-webkit-scrollbar-thumb {
    background-color: var(--color-text-half);
    border-radius: 3px;
    cursor: pointer;
}

.smallPhotoContainer::-webkit-scrollbar-track {
    background-color: var(--color-background);
}

.smallPhoto {
    position: relative;
    /* margin-bottom: 5px; */
    transition: all 0.3s ease;
}

.bigPhoto .img,
.smallPhoto .img {
    height: 100%;
    position: absolute;

}

.bigPhoto .img {
    width: 100%;
}

.bigPhoto .img img {
    height: 100%;
    width: auto;
    margin: 0 50%;
    transform: translateX(-50%);
    border-width: 3px;
    border-style: solid;
    border-image: linear-gradient(to right, #ff6b6b, #ffb677, #fdff94, #d4fc79, #96e6a1);
    border-image-slice: 1;

}

.smallPhoto .img img {
    height: 100%;
    width: auto;

}

.fouceStyle {
    position: absolute;
    height: 100%;
    border-width: 3px;
    border-style: solid;
    border-image: linear-gradient(to right, #ff6b6b, #ffb677, #fdff94, #d4fc79, #96e6a1);
    border-image-slice: 1;
    z-index: 999;
    transition: all .3s ease-in-out;
    animation: hueRotate 5s infinite linear;
}

@keyframes hueRotate {
    100% {
        filter: hue-rotate(360deg)
    }
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
    opacity: 0;
}

.arrLeft {
    opacity: 0;
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
</style>