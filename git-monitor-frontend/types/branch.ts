export default interface Branch {
  id: number
  name: string
  branch_type: string
  is_head: boolean
  upstream?: string
  repoDir: string
}
