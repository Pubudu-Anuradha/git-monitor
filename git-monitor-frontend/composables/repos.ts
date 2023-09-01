import api from './api'

const getAllRepos = async () => {
  const response = await api.get<{ Ok: Repository[] }>('/repos')
  return response.data
}

const updateAllRepos = async () => {
  const response = await api.post<string[]>('/repos/update')
  return response.data.length > 0
}

const updateRepo = async (
  path: string,
  absolute: boolean,
  managed: boolean,
) => {
  const response = await api.post<{ Ok: Repository }>('/repos/repo', {
    absolute,
    managed,
    path,
  })
  return response.data
}

export default { getAllRepos, updateAllRepos, updateRepo }
