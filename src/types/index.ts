export interface Picture {
  id: number
  url: string
  title: string
  srcs: { src: string; aspect_ratio: number }[]
  star: number
  collect: boolean
  download: boolean
  deleted: boolean
}

export interface MasonryReturn {
  cols: number[]
  mrLeft: number
  margin: number
}

export interface LoadedReturn {
  width: number
  height: number
  index: number
}

export type EventHandler = {
  emit: (eventName: string, eventData: any) => void
}
