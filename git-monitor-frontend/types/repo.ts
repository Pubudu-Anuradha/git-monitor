import Branch from "./branch"
import type { RepoState } from "./repostate"
import FileStatus from "./status"

export default interface Repo {
  dir: string
  state: RepoState
  name: string
  is_valid: boolean
  managed: boolean
  updatedAt: Date
  statuses: FileStatus[]
  branches: Branch[]
}
