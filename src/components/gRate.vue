<script setup lang="ts">
import { computed, h, ref } from 'vue'
type SizeType = 'tiny' | 'small' | 'medium' | 'large';
const props = defineProps({
    count: {
        type: Number,
        required: false,
        default: 5
    },
    color: {
        type: String,
        required: false,
        default: '#f88'
    },
    size: {
        type: [Number, String as () => SizeType],
        required: false,
        default: 24
    },
    value: {
        type: Number,
        required: false,
        default: 0
    },
    id:{
        type:String,
        required:false,
        default: '1'
    }
})

const emit = defineEmits(['update'])

const sizeMap = {
    tiny: '16px',
    small: '24px',
    medium: '32px',
    large: '48px'
}
const size = computed(() => {
    return (typeof props.size === 'number') ? `${props.size}px` : sizeMap[props.size]
})

const count = computed(() => typeof props.count === 'number' ? props.count : 5);
const model = ref(props.value);
const IconStar = h('svg', {
    fill: props.color,
    xmlns: "http://www.w3.org/2000/svg",
    viewBox: "0 0 512 512"
}, h('path', {
    d: "M394 480a16 16 0 0 1-9.39-3L256 383.76L127.39 477a16 16 0 0 1-24.55-18.08L153 310.35L23 221.2a16 16 0 0 1 9-29.2h160.38l48.4-148.95a16 16 0 0 1 30.44 0l48.4 149H480a16 16 0 0 1 9.05 29.2L359 310.35l50.13 148.53A16 16 0 0 1 394 480z"
}))

// const value = computed(()=>model.value>props.count?0:model.value);
// const currentIndex = ref(model.value)
const oldIndex = ref(props.value)
const scaleSize = computed(() => 0.95 * (+size.value.split('px')[0]))

function click(n: number) {
    model.value = n
    emit('update', {id:props.id,value:n})
    // emit('update', n + 1)
    // model.value = n + 1
    oldIndex.value = n
}
function mousemove(n: number) {
    oldIndex.value = n
}
function mouseout() {
    oldIndex.value = model.value
}

</script>

<template>
    <div class="rateBox">
        <div class="rate" :style="{ width: size, height: size }" v-for="i in count" :key="i">
            <icon-star :fill="oldIndex < i ? 'gray' : props.color" :class="oldIndex === i ? 'actClass' : null"
                :width="oldIndex < i ? scaleSize : size" :height="oldIndex < i ? scaleSize : size" @click.stop="click(i)"
                @mousemove="mousemove(i)" @mouseout="mouseout()" />
        </div>

    </div>
</template>

<style scoped>
.rateBox {
    display: flex;
    margin-bottom: 2px;
    padding: 0 8px;
    width: 125px;
    border-radius: 20px;
    background: linear-gradient(to right, var(--backlinear1), var(--backlinear2));
    backdrop-filter: blur(10px);
}

.rate {
    margin: 2px;
    display: flex;
    justify-content: center;
    cursor: pointer;
}

svg {
    transition: all 0.3s;
}

svg.actClass {
    animation: scale 0.34s alternate;
}

@keyframes scale {
    0% {
        transform: scale(1);
    }

    25% {
        transform: scale(0.9);
    }

    50% {
        transform: scale(1.2);
    }

    100% {
        transform: scale(1);
    }
}</style>