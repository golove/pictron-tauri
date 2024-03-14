<script setup lang="ts">
import { ref, onMounted, watchEffect, h, type VNode } from 'vue'
import { findMax, findMin } from '@/utils'
import { useCounterStore } from '@/stores/counter';
import { storeToRefs } from 'pinia';

const store = useCounterStore()
const { mainWidth } = storeToRefs(store)
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
const slots = defineSlots<{
    default: () => any[]
}>();


const count = ref(0)
watchEffect(() => {
    count.value += 1
    itemWidth.value = (mainWidth.value - props.gap * (props.cols + 1)) / props.cols
    init()
    masonry.value = h('div', { style: { position: 'relative' } }, [
        children.value.map((e, i) => {
            const min = findMin(cols.value);
            const height = itemWidth.value / e.props!.aspectRatio
            const left = itemWidth.value * min + props.gap * (min + 1)
            const x = ref('0px'), y = ref('0px')
           
            const childrenList = h(e, {
                style: {
                    width: itemWidth.value + 'px', height: height + 'px', left: left + 'px', top: cols.value[min] + 'px',
                    borderRadius: '10px', '--x': x.value, '--y': y.value
                }
            })
            onmousemove = (ev: MouseEvent) => {
                if (e.el) {
                    console.log(ev.clientX, ev.clientY)
                    const rect = e.el.getBoundingClientRect()
                    x.value = (ev.clientX - rect.left) + 'px'
                    y.value = (ev.clientY - rect.top) + 'px'
                    console.log(x.value, y.value)
                }
            }
            cols.value[min] += (height + props.gap)
            if (i === children.value.length - 1) {
                const max = findMax(cols.value);
                masonryRef.value.style.height = cols.value[max] + 'px'
            }
           
            return childrenList
        })
    ])
    console.log('run count', count.value)
});




onMounted(() => {
    init()
})


function init() {
    if (!masonryRef.value) return
    cols.value = new Array(props.cols).fill(props.gap)
    defaultVNode.value = slots.default && slots.default()
    if (defaultVNode.value && defaultVNode.value[0] && defaultVNode.value[0].children) {
        children.value = defaultVNode.value[0].children as VNode[]
    }
}

</script>
<template>

    <masonry ref="masonryRef"></masonry>
</template>