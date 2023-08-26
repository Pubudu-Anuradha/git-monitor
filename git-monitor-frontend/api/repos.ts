import { Repository } from 'types/repo'
import { apiInstance } from './api'

export const get_all_repos = async () => {
  const response = await apiInstance.get<{ Ok: Repository[] }>('/repos')
  return response.data
}

export const update_all_repos = async () => {
  const response = await apiInstance.post<string[]>('/repos/update')
  return response.data.length > 0
}

export const update_repo = async (
  path: string,
  absolute: boolean,
  managed: boolean
) => {
  const response = await apiInstance.post<{Ok:Repository}>('/repos/repo', {
    path,
    absolute,
    managed,
  })
  return response.data
}