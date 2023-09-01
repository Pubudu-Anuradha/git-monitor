declare global {
  type RepoState =
    | 'ApplyMailbox'
    | 'ApplyMailboxOrRebase'
    | 'Bisect'
    | 'CheeryPickSequence'
    | 'CherryPick'
    | 'Clean'
    | 'Merge'
    | 'Rebase'
    | 'RebaseInteractive'
    | 'RebaseMerge'
    | 'Revert'
    | 'RevertSequence'

  type Status =
    | 'CONFLICTED'
    | 'CURRENT'
    | 'IGNORED'
    | 'INDEX_DELETED'
    | 'INDEX_MODIFIED'
    | 'INDEX_NEW'
    | 'INDEX_RENAMED'
    | 'INDEX_TYPECHANGE'
    | 'WT_DELETED'
    | 'WT_MODIFIED'
    | 'WT_NEW'
    | 'WT_RENAMED'
    | 'WT_TYPECHANGE'

  interface Branch {
    branch_type: string
    id: number
    is_head: boolean
    name: string
    repoDir: string
    upstream?: string
  }

  interface FileStatus {
    id: number
    path: string
    status: Status
  }

  interface Repository {
    branches: Branch[]
    dir: string
    is_valid: boolean
    managed: boolean
    name: string
    state: RepoState
    statuses: FileStatus[]
    updatedAt: Date
  }
}

export {};