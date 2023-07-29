<template>
  <div>
    <h1>Color mode: {{ $colorMode.value }}</h1>
    <select
      v-model="$colorMode.preference"
      class="p-2 rounded-md dark:bg-stone-800 dark:text-stone-100 border-neutral-400 outline-neutral-400"
    >
      <option value="system">System</option>
      <option value="light">Light</option>
      <option value="dark">Dark</option>
    </select>
  </div>
</template>
