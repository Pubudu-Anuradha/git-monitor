import type { Repository } from '~/types/repo'

import { apiInstance } from './api'

export const getAllRepos = async () => {
  const response = await apiInstance.get<{ Ok: Repository[] }>('/repos')
  return response.data
}

export const updateAllRepos = async () => {
  const response = await apiInstance.post<string[]>('/repos/update')
  return response.data.length > 0
}

export const updateRepo = async (
  path: string,
  absolute: boolean,
  managed: boolean,
) => {
  const response = await apiInstance.post<{ Ok: Repository }>('/repos/repo', {
    absolute,
    managed,
    path,
  })
  return response.data
}
