<template>


        <div v-if="loading" class="loading" :style="props.width"></div>
        <img v-else :src="props.src" alt="">


</template>
<script setup lang="ts">
import { ref } from 'vue'
const props = defineProps({
    src: {
        type: String,
        require: true,
        default:""
    },
    width: {
        type: String,
        require: true,
        default:'1px'
    }

})


const loading = ref(false)
const img = new Image()
img.src = props.src
loading.value = true
img.onload = () => {
    loading.value = false
}

// const width = computed(()=>)



</script>

<style>
   .loading{
    height: 100%; 
    display: flex;
    justify-content: center; 
    align-items: center;
    font-size: 12px;
    background-color: antiquewhite;
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