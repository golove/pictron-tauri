export interface Picture {
  _id: string
  id: number
  url: string
  title: string
  srcs: { src: string; aspect_ratio: number }[]
  star: number
  collect: boolean
  download: boolean
  delete: boolean
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
