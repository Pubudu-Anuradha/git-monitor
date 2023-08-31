export type Status =
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

export interface FileStatus {
  id: number
  path: string
  status: Status
}
