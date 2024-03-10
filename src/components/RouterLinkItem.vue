

<template>
    <div class="item">
        <i><slot name="icon"></slot></i>
        
        <div class="title"> 
            <h3><slot name="title"></slot></h3>
        </div>
    </div>

</template>

<style>
.item {
  display: flex;
  justify-content: center;
  position: relative;
}
.title {
  flex: 1;
  margin-left: 0rem;
}
i {
  display: flex;
  place-items: center;
  place-content: center;
  width: 32px;
  height: 32px;
  color: var(--color-text);
}
h3 {
  font-size: 1.2rem;
  font-weight: 500;
  margin-bottom: 0.4rem;
  color: var(--color-heading);
}

</style>