<template>
  <span class="red">
    <slot />
  </span>
</template>

<style scoped>
.red {
  color: maroon;
}
.dark-mode .red {
  color: red;
}
</style>
