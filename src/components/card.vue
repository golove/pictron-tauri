<template>
  <div class="card">
    <img :src="props.img.src" />
  </div>
</template>

<script setup lang="ts">

 const props = defineProps({
    img: {
      type: Object,
      required: true
    }
  })

const height = 230 / (props.img.aspect_ratio)

</script>
<style>
.card{
    width: 230px;
    height: v-bind(height+'px');
}
.card img{
    width: 100%;
    height: auto;
}
</style>
