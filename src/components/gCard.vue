<script setup lang="ts">
import { computed, ref } from 'vue'

const props = defineProps({
    src: {
        type: String,
        require: true,
        default: ''
    },
    width: {
        type: [Number, String],
        require: true
    },
    height: {
        type: [Number, String],
        require: false
    },
    absolute: {
        type: Boolean,
        require: false,
        default: false
    },
    borderRadius: {
        type: [String, Number],
        require: false,
        default: 0
    }
})

const loading = ref(false)
const img = new Image()
img.src = props.src
loading.value = true
img.onload = () => {
    loading.value = false
}

const slots = defineSlots()
const showheader = computed(()=>slots.header===undefined?false:true)
const showtoolkit = computed(()=>slots.toolkit===undefined?false:true)

const width = computed(() =>
    typeof props.width === 'string' ? props.width : props.width + 'px'
)
const height = computed(() =>
    typeof props.height === 'string' ? props.height : props.height + 'px'
)
const borderRadius = computed(() =>
    typeof props.borderRadius === 'string' ? props.borderRadius : props.borderRadius + 'px'
)



</script>
<template>
    <div class="card" :style="{
        width: width,
        height: height,
        position: props.absolute ? 'absolute' : 'relative',
        borderRadius: borderRadius
    }">
        <div v-if="loading" class="loading">loading...</div>
        <template v-else>
            <img :src="props.src" alt="">

            <h3 v-if="showheader" class="title">
                <slot name="header"></slot>
            </h3>

            <div v-if="showtoolkit" class="toolBox">
              <slot name="toolkit"></slot>
            </div>
        </template>
    </div>
</template>
<style>
.card {
    background: rgba(255, 37, 102, 0.626);
    overflow: hidden;
    animation: cardscal 0.5s ease-in-out alternate;
    -webkit-animation: cardscal 0.3s ease-in-out alternate;
    transition: all 0.3s ease-in-out;
    -webkit-transition: all 0.3s ease-in-out;
    box-shadow: 0 0 10px rgba(0, 0, 0, 0.3);
    --backlinear1: rgba(122, 83, 103, 0.3);
    --backlinear2: rgba(74, 114, 67, 0.3);
}

@keyframes cardscal {
    from {
        opacity: 0;
        transform: scale(0.8);
    }

    to {
        opacity: 1;
        transform: scale(1);
    }
}

.card h3.title {
    position: absolute;
    font-size: 14px;
    font-weight: bold;
    bottom: 50px;
    width: 55%;
    height: 48px;
    overflow: hidden;
    line-height: 18px;
    color: rgba(252, 242, 252);
    transform: translateY(0);
    z-index: 5;
    padding: 6px 8px;
    /* 省略超出字符 */
    text-overflow: ellipsis;
    transition: all 3s ease;
    background-size: 200% 100%;

}

h3.title::before {
    content: '';
    display: block;
    position: absolute;
    top: 0;
    right: 0;
    bottom: 0;
    left: 0;
    background-color: inherit;
    mix-blend-mode: difference;
    pointer-events: none;
}

@keyframes gradientAnimation {

    100% {
        filter: hue-rotate(360deg);
    }
}

.card::before {
    content: '';
    position: absolute;
    width: 120%;
    height: 120%;
    transform: translate(-50%, -50%);
    background: radial-gradient(closest-side circle, rgba(40, 255, 11, 0.9),
            rgba(0, 0, 0, 0));
    animation: gradientAnimation 5s infinite alternate;
    z-index: 2;
    border-radius: inherit;
    left: var(--x, -1000px);
    top: var(--y, -1000px);
}

.card img {
    position: absolute;
    inset: 2px;
    width: calc(100% - 4px);
    height: calc(100% - 4px);
    transition: transform 0.3s ease;
    border-radius: inherit;
    z-index: 3;
    overflow: hidden;
}

.card .toolBox {
    position: absolute;
    left: 6px;
    bottom: 8px;
    z-index: 9;
}


.loading{
    height: 100%; 
    display: flex;
    justify-content: center; 
    align-items: center;
    font-size: 12px;
}
.loading::before{
    content: "";
    position: absolute;
    width: 70px;
    height: 70px;
    border:solid rgb(209, 132, 185) 4px;
    border-radius: 50%;
    border-top: 4px solid #4be633;
    animation: spin 1s linear infinite;
}

@keyframes spin {
      0% { transform: rotate(0deg); }
      100% { transform: rotate(360deg); }
    }
</style>