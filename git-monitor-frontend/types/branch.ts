export interface Branch {
  branch_type: string
  id: number
  is_head: boolean
  name: string
  repoDir: string
  upstream?: string
}
