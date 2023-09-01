<script setup lang="ts">
import { getAllRepos, updateRepo as uR, updateAllRepos } from '~/api/repos'
import { Repository } from '~/types/repo'

const repoData = ref<Repository[]>([])

const fetchRepos = async () => {
  const response = await getAllRepos()
  response.Ok.forEach((repo) => {
    // eslint-disable-next-line no-console
    console.log(
      repo.name,
      new Date(repo.updatedAt).toLocaleDateString('en-US', {
        day: '2-digit',
        hour: '2-digit',
        minute: '2-digit',
        month: '2-digit',
        year: 'numeric',
      }),
    )
  })
  const reg = /^.*git-monitor.*$/

  repoData.value = response.Ok.filter(
    (repo) => repo.is_valid && reg.test(repo.dir),
  )
}

const loading = ref(false)
const updateRepos = async () => {
  loading.value = true
  await updateAllRepos()
  loading.value = false
  await fetchRepos()
}

const updateRepo = async (repo: Repository, managed: boolean | null) => {
  loading.value = true
  const result = (
    await uR(repo.dir, true, managed === null ? repo.managed : managed)
  ).Ok
  // eslint-disable-next-line no-console
  console.log(result)
  repoData.value = repoData.value.map((r) => {
    if (r.dir === repo.dir) {
      r = result
    }
    return r
  })
  loading.value = false
}
</script>

<template>
  <div>
    <h1>Welcome to the test page</h1>
    <div class="flex flex-col gap-2">
      <v-btn @click="fetchRepos">Fetch repos</v-btn>
      <v-btn @click="updateRepos">Update repos</v-btn>
      <span> Loading: {{ loading }} </span>
    </div>
    <div v-for="repo in repoData" :key="repo.dir" class="repos">
      <repo-card :repo="repo" :updater="updateRepo" />
    </div>
  </div>
</template>

<style lang="postcss" scoped>
.repos {
  @apply mb-4;
}
</style>
