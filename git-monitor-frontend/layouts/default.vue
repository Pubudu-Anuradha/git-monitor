<template>
  <div class="flex justify-center bg-stone-200 dark:bg-stone-600">
    <div
      class="grow p-4 shadow-xl max-w-screen-xl bg-stone-50 dark:bg-stone-950 dark:text-stone-100 min-h-screen"
    >
      <h1 class="text-4xl">
        <span class="text-red-600">G</span>it
        <span class="text-red-600">M</span>onitor
      </h1>
      <color-mode-picker></color-mode-picker>
      <ul>
        <li><nuxt-link to="/">Home</nuxt-link></li>
        <li><nuxt-link to="/about">About</nuxt-link></li>
        <li><nuxt-link to="/test">Test</nuxt-link></li>
      </ul>
      <slot />
    </div>
  </div>
</template>

<script>
definePageMeta({
  colorMode: 'light',
})
</script>
