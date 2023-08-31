import { Status } from '~/types/status'

export const statusToString = (status: Status) => {
  let res
  switch (status) {
    case 'CURRENT':
      res = 'Current'
      break
    case 'INDEX_NEW':
      res = 'New'
      break
    case 'INDEX_MODIFIED':
      res = 'Modified'
      break
    case 'INDEX_DELETED':
      res = 'Deleted'
      break
    case 'INDEX_RENAMED':
      res = 'Renamed'
      break
    case 'INDEX_TYPECHANGE':
      res = 'Typechange'
      break
    case 'WT_NEW':
      res = 'New'
      break
    case 'WT_MODIFIED':
      res = 'Modified'
      break
    case 'WT_DELETED':
      res = 'Deleted'
      break
    case 'WT_TYPECHANGE':
      res = 'Typechange'
      break
    case 'WT_RENAMED':
      res = 'Renamed'
      break
    case 'IGNORED':
      res = 'Ignored'
      break
    case 'CONFLICTED':
      res = 'Conflicted'
      break
    default:
      res = 'Unknown'
  }
  return res
}
