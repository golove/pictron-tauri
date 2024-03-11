<script setup lang="ts">
import { ref, onMounted, watchEffect, h, type VNode, watch } from 'vue'
import { findMax, findMin } from '@/utils'
import { useCounterStore } from '@/stores/counter';
const store = useCounterStore()

const props = defineProps({
    cols: {
        type: Number,
        default: 3
    },
    gap: {
        type: Number,
        default: 8
    }
})
const defaultVNode = ref<VNode[]>([])
const children = ref<VNode[]>([]);
const masonry = ref()
const masonryRef = ref()
const itemWidth = ref()
const cols = ref(new Array(props.cols).fill(props.gap))
const lightBorder = ref<{ x: string, y: string }[]>([])
const slots = defineSlots<{
    default: () => any[]
}>();


watchEffect(() => {
    init()
});





masonry.value = h('div', { class: 'masonry' }, [...defaultVNode.value])
onmousemove = (e: MouseEvent) => {
    setLightBorder(e)
}
onMounted(() => {
    init()

})

function setItemStyle(width: number) {
    if (children.value && children.value.length > 0) {
        for (let index = 0; index < children.value.length; index++) {
            const el = children.value[index];
            const min = findMin(cols.value);
            const height = width / el.props!.aspectRatio
            const left = itemWidth.value * min + props.gap * (min + 1)
            const border = lightBorder.value[index]
            el.props!.style = `position:absolute;width:${width}px;
                  height:${height}px;
                  left:${left}px;top:${cols.value[min]}px;
                  background: #a99;border-radius: 10px;margin-bottom: 10px;
                  --x:${border ? border.x : '0px'};--y:${border ? border.y : '0px'}
                 `
            cols.value[min] += (height + props.gap)
            if (index === children.value.length - 1) {
                const max = findMax(cols.value);
                masonryRef.value.style.height = cols.value[max] + 'px'
            }
        }
    }
}

function setLightBorder(e: MouseEvent) {
    if (children.value.length === 0) return
    for (let index = 0; index < children.value.length; index++) {
        const s = children.value[index];
        if (s.el) {
            const rect = s.el.getBoundingClientRect()
            const x = e.clientX - rect.left
            const y = e.clientY - rect.top
            lightBorder.value[index] = {
                x: x + 'px',
                y: y + 'px'
            }
        }
    }
}

function init() {
    if (!masonryRef.value) return
    cols.value = new Array(props.cols).fill(props.gap)

    itemWidth.value = (store.mainWidth - props.gap * (props.cols + 1)) / props.cols
    defaultVNode.value = slots.default && slots.default()
    if (defaultVNode.value && defaultVNode.value[0] && defaultVNode.value[0].children) {
        children.value = defaultVNode.value[0].children as VNode[]
    }
    setItemStyle(itemWidth.value)
    masonry.value = h('div', { style: { position: 'relative', width: '100%' } }, [...defaultVNode.value])

}

</script>
<template>
    <masonry ref="masonryRef"></masonry>
</template>