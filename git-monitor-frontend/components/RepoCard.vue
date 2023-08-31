<script setup lang="ts">
import { Repository } from '~/types/repo'
import { statusToString } from '~/utils/statusToString'

defineProps<{
  repo: Repository
  updater: (repo: Repository, managed: boolean | null) => Promise<void>
}>()
</script>

<template>
  <div
    class="max-w-xl overflow-hidden p-4 rounded-md shadow-md bg-stone-100 dark:bg-stone-800 flex flex-col gap-2"
  >
    <div class="flex justify-between align-middle">
      <h1 class="font-bold text-xl italic">
        {{ repo.name }} : {{ repo.state }}
      </h1>
      <button @click="updater(repo, !repo.managed)">
        toggle monitoring with git-monitor
      </button>
    </div>
    <div>
      <div>branches: {{ repo.branches.length }}</div>
      <div>changes?: {{ repo.statuses.length }}</div>
      <div>managed?: {{ repo.managed }}</div>
    </div>
    <div class="text-right">
      {{ repo.dir }}
    </div>
    <ul>
      <li v-for="branch in repo.branches" :key="branch.id">
        {{ branch.branch_type }} : {{ branch.name }}
        <span v-if="branch.is_head" class="text-red-500">HEAD</span>
      </li>
    </ul>
    <h2>Working tree changes</h2>
    <ul>
      <li
        v-for="status in repo.statuses.filter((s) =>
          s.status.startsWith('WT_'),
        )"
        :key="status.path"
        class="grid grid-cols-2 grid-flow-col"
      >
        <span>
          {{ statusToString(status.status) }}
        </span>
        <span>
          {{ status.path }}
        </span>
      </li>
    </ul>
    <h2>Index changes(Staged)</h2>
    <ul>
      <li
        v-for="status in repo.statuses.filter((s) =>
          s.status.startsWith('INDEX_'),
        )"
        :key="status.path"
        class="grid grid-cols-2 grid-flow-col"
      >
        <span>
          {{ statusToString(status.status) }}
        </span>
        <span>
          {{ status.path }}
        </span>
      </li>
    </ul>
    <h2>Other changes</h2>
    <ul>
      <li
        v-for="status in repo.statuses.filter(
          (s) => !s.status.startsWith('WT_') && !s.status.startsWith('INDEX_'),
        )"
        :key="status.path"
        class="grid grid-cols-2 grid-flow-col"
      >
        <span>
          {{ statusToString(status.status) }}
        </span>
        <span>
          {{ status.path }}
        </span>
      </li>
    </ul>
    <button @click="updater(repo, null)">Update</button>
  </div>
</template>

<style scoped>
button {
  border: 1px solid black;
  border-radius: 0.25rem;
  padding: 0.25rem;
  background-color: white;
  color: black;
  cursor: pointer;
}
h2 {
  margin-top: 1rem;
  font-size: large;
  margin-bottom: 0.5rem;
}
</style>
