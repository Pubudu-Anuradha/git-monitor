import { Branch } from "./branch"
import type { RepoState } from "./repoState"
import { FileStatus } from "./status"

export interface Repository {
  dir: string
  state: RepoState
  name: string
  is_valid: boolean
  managed: boolean
  updatedAt: Date
  statuses: FileStatus[]
  branches: Branch[]
}