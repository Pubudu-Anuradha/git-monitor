<script setup lang="ts">
defineProps<{
  repo: Repository
  updater: (repo: Repository, managed: boolean | null) => Promise<void>
}>()

const remoteHidden = ref(false)

const toggleRemoteVisible = () => {
  remoteHidden.value = !remoteHidden.value
}
</script>

<template>
  <v-card>
    <v-card-title class="flex items-center w-full justify-between">
      {{ repo.name }} : {{ repo.state }}
    </v-card-title>
    <v-card-subtitle>
      <div class="text-right">
        {{ repo.dir }}
      </div>
    </v-card-subtitle>
    <v-card-item class="flex flex-col justify-center">
      <div class="inline-block mx-4">branches: {{ repo.branches.length }}</div>
      <div class="inline-block mx-4">changes?: {{ repo.statuses.length }}</div>
      <div class="inline-block mx-4">managed?: {{ repo.managed }}</div>
    </v-card-item>
    <v-card-item>
      <v-container fluid>
        <v-row :no-gutters="true">
          <v-col sm="12" md="6">
            <span class="text-red-600">HEAD</span> @branch:
            {{ repo.branches.find((b) => b.is_head)?.name || 'Unknown' }}
            <v-list
              v-for="branch_type in ['Local', 'Remote']"
              :key="branch_type"
              :title="branch_type + 'Branches'"
              :style="{
                display:
                  branch_type === 'Local'
                    ? 'block'
                    : remoteHidden
                    ? 'none'
                    : 'block',
              }"
            >
              <h2>{{ branch_type }} Branches</h2>
              <v-list-item
                v-for="branch in repo.branches.filter(
                  (b) => b.branch_type === branch_type,
                )"
                :key="branch.id"
              >
                <v-list-item-title>
                  <span v-if="branch.is_head" class="text-red-500">HEAD</span>
                  {{ branch.name }}
                </v-list-item-title>
                <v-list-item-subtitle v-if="branch.upstream">
                  {{ branch.upstream }}
                </v-list-item-subtitle>
              </v-list-item>
            </v-list>
            <v-btn variant="outlined" @click="toggleRemoteVisible()">
              {{ remoteHidden ? 'Show' : 'Hide' }} Remote Branches
            </v-btn>
          </v-col>
          <v-col sm="12" md="6">
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
            <h2 class="ech2">Other changes</h2>
            <ul>
              <li
                v-for="status in repo.statuses.filter(
                  (s) =>
                    !s.status.startsWith('WT_') &&
                    !s.status.startsWith('INDEX_'),
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
          </v-col>
        </v-row>
      </v-container>
    </v-card-item>
    <v-card-actions style="display: flex; width: 100%; justify-content: center">
      <v-btn variant="outlined" @click="updater(repo, null)">Update</v-btn>
      <v-btn variant="outlined" @click="updater(repo, !repo.managed)"
        >toggle monitoring</v-btn
      >
    </v-card-actions>
  </v-card>
</template>

<style scoped>
h2 {
  font-weight: 700;
  margin-top: 1rem;
  font-size: large;
  margin-bottom: 0.5rem;
}
</style>
