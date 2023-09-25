const getDefaultConfig = async () => {
  return (await api.get<GitConfig>('/config/get')).data
}

const getUpdatedDefaultConfig = async () => {
  return (await api.put<GitConfig>('/config/update')).data
}

export default { getDefaultConfig, getUpdatedDefaultConfig }
