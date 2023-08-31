import type { RepoState } from './repoState'

import { Branch } from './branch'
import { FileStatus } from './status'

export interface Repository {
  branches: Branch[]
  dir: string
  is_valid: boolean
  managed: boolean
  name: string
  state: RepoState
  statuses: FileStatus[]
  updatedAt: Date
}
