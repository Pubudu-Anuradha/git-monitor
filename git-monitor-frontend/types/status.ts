export type Status =
  | 'CURRENT'
  | 'INDEX_NEW'
  | 'INDEX_MODIFIED'
  | 'INDEX_DELETED'
  | 'INDEX_RENAMED'
  | 'INDEX_TYPECHANGE'
  | 'WT_NEW'
  | 'WT_MODIFIED'
  | 'WT_DELETED'
  | 'WT_TYPECHANGE'
  | 'WT_RENAMED'
  | 'IGNORED'
  | 'CONFLICTED'

export interface FileStatus {
  id: number
  status: Status
  path: string
}
