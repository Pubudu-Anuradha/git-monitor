declare global {
  interface ConfigEntry {
    id: number
    name: string
    value: string
  }

  interface GitConfig {
    entries: ConfigEntry[]
  }
}

export {};