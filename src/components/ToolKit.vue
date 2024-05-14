<script setup lang="ts">

import { computed, h, ref,watch } from 'vue'
// import { listen } from '@tauri-apps/api/event'
const heart = h(
    'svg',
    {
        xmlns: 'http://www.w3.org/2000/svg',
        xlink: 'http://www.w3.org/1999/xlink',
        viewBox: '0 0 512 512'
    },
    h('path', {
        d: 'M256 448a32 32 0 0 1-18-5.57c-78.59-53.35-112.62-89.93-131.39-112.8c-40-48.75-59.15-98.8-58.61-153C48.63 114.52 98.46 64 159.08 64c44.08 0 74.61 24.83 92.39 45.51a6 6 0 0 0 9.06 0C278.31 88.81 308.84 64 352.92 64c60.62 0 110.45 50.52 111.08 112.64c.54 54.21-18.63 104.26-58.61 153c-18.77 22.87-52.8 59.45-131.39 112.8a32 32 0 0 1-18 5.56z'
    })
)
const downloadV = h(
    'svg',
    {
        xmlns: 'http://www.w3.org/2000/svg',
        xlink: 'http://www.w3.org/1999/xlink',
        viewBox: '0 0 24 24'
    },
    h('path', {
        d: 'M16.59 9H15V4c0-.55-.45-1-1-1h-4c-.55 0-1 .45-1 1v5H7.41c-.89 0-1.34 1.08-.71 1.71l4.59 4.59c.39.39 1.02.39 1.41 0l4.59-4.59c.63-.63.19-1.71-.7-1.71zM5 19c0 .55.45 1 1 1h12c.55 0 1-.45 1-1s-.45-1-1-1H6c-.55 0-1 .45-1 1z'
    })
)

const trash = h(
    'svg',
    {
        xmlns: 'http://www.w3.org/2000/svg',
        xlink: 'http://www.w3.org/1999/xlink',
        viewBox: '0 0 24 24'
    },
    h('path', {
        d: 'M6 19c0 1.1.9 2 2 2h8c1.1 0 2-.9 2-2V9c0-1.1-.9-2-2-2H8c-1.1 0-2 .9-2 2v10zM18 4h-2.5l-.71-.71c-.18-.18-.44-.29-.7-.29H9.91c-.26 0-.52.11-.7.29L8.5 4H6c-.55 0-1 .45-1 1s.45 1 1 1h12c.55 0 1-.45 1-1s-.45-1-1-1z'
    })
)

const DownloadDone = h(
    'svg',
    {
        xmlns: 'http://www.w3.org/2000/svg',
        xlink: 'http://www.w3.org/1999/xlink',
        viewBox: '0 0 24 24'
    },
    h('path', { d: 'M5 18h14v2H5v-2zm4.6-2.7L5 10.7l2-1.9l2.6 2.6L17 4l2 2l-9.4 9.3z' })
)
const props = defineProps({
    id: {
        type: Number,
    }, like: {
        type: Boolean,
        default: false
    },
    download: {
        type: Boolean,
        default: false
    },
    deleted: {
        type: Boolean,
        default: false
    },
    length: {
        type: Number,
        default: 0
    },
    downloadCount:{
        type:Number,
        default:0
    }

})
const emit = defineEmits(['update'])
const fill = '#ddd'
const actFill = '#f88'
const downloadFlag = ref(props.download)
// const texts = ref([0, 0, 0])
const flags = ref([props.like, props.download, props.deleted])




let dCount = computed(() => props.downloadCount)

function click(type: string, n: number) {

    flags.value[n] = !flags.value[n]
    emit('update', { type, id: props.id, value: flags.value[n] })
    // console.log(key,newValue,value)
    if (n === 0 && flags.value[n]) {

        if (flags.value[2]) {
            console.log('deleted')
            click('deleted', 2)
        }
    }
    if (n === 2 && flags.value[n]) {
        if (flags.value[0]) {
            console.log('collect', flags.value[n], n)
            click('collect', 0)
        }
    }
    if(n ===1 && !flags.value[n]){
        downloadFlag.value = flags.value[n]
    }
   
}

watch(()=>dCount.value, (newVal) => {
        if (newVal+1 > props.length) {
            downloadFlag.value = true
        }
})


</script>
<template>
    <div class="toolkit">
        <div class="toolkitItem" @click.stop="click('collect', 0)">
            <heart :fill="flags[0] ? actFill : fill" width='18px' height='18px' />
            <!-- <div>{{ texts[0] }}</div> -->
        </div>
        <div class="toolkitItem" @click.stop="click('download', 1)">
            <DownloadDone v-if="downloadFlag" :fill="actFill"  width='20px' height='20px' />
            <downloadV v-else :fill="fill" width='20px' height='20px' />
            <div>{{ downloadFlag ? '' : dCount ? dCount : '' }}</div>
        </div>
        <div class="toolkitItem" @click.stop="click('deleted', 2)">
            <trash :fill="flags[2] ? actFill : fill" width='20px' height='20px' />
        </div>
    </div>
</template>
<style>
.toolkit {
    position: relative;
    height: 24px;
    width: 100%;
    background: linear-gradient(to right, rgba(255, 34, 148, 0.3), rgba(61, 220, 27, 0.3));
    backdrop-filter: blur(10px);
    border-radius: 20px;
    padding: 2px 8px;
    display: flex;
    align-items: center;
    justify-content: center;
}

.toolkitItem {
    width: 42px;
    height: 24px;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    color: aliceblue;
}

svg:hover {
    fill: #f88;
}
</style>