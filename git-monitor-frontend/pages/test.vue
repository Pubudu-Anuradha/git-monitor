<script setup lang="ts">
import { Repository } from '~/types/repo'
import { get_all_repos, update_all_repos, update_repo } from '~/api/repos'

let repoData = ref<Repository[]>([])

const fetchRepos = async () => {
  const response = await get_all_repos()
  response.Ok.forEach((repo) => {
    console.log(
      repo.name,
      new Date(repo.updatedAt).toLocaleDateString('en-US', {
        day: '2-digit',
        month: '2-digit',
        year: 'numeric',
        hour: '2-digit',
        minute: '2-digit',
      }),
    )
  })
  const reg = new RegExp(/^.*git-monitor.*$/)

  repoData.value = response.Ok.filter(
    (repo) => repo.is_valid && reg.test(repo.dir),
  )
}

let loading = ref(false)
const updateRepos = async () => {
  loading.value = true
  await update_all_repos()
  loading.value = false
  await fetchRepos()
}

const updateRepo = async (repo: Repository,managed: boolean | null) => {
  loading.value = true
  const result = (await update_repo(repo.dir, true, managed === null ? repo.managed : managed)).Ok
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
      <button @click="fetchRepos">Fetch repos</button>
      <button @click="updateRepos">Update repos</button>
      <span> Loading: {{ loading }} </span>
    </div>
    <div v-for="repo in repoData" class="repos">
      <Repo :repo="repo" :updater="updateRepo" />
    </div>
  </div>
</template>

<style lang="postcss" scoped>
.repos {
  @apply mb-4;
}
</style>
