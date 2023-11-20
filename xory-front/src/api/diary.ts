import { useGet } from '@/request'

export interface DiaryListReq {
  uid: number
  page_number: number
  page_size: number
  keywords: string
  tags: string
}

export interface DiaryListRes {
  title?: string
  content?: string
  did?: number
  tags?: number[]
  date?: Date | string
}

export const diaryList = async (diaryListReq: DiaryListReq) => {
  const { data, execute } = useGet<DiaryListRes[]>('diary/list', diaryListReq)
  await execute()
  return data!
}

export interface DiaryDetailReq {
  did: number
}

export interface DiaryDetailRes {
  did?: number
  date?: Date | string
  title?: string
  content?: string
  temperature?: number
  weather?: string
  mood?: string
  tags?: number[] | string
  date_create?: Date | string
  date_modify?: Date | string
  uid?: number
  longitude?: number
  latitude?: number
}

export const diaryDetail = async (diaryDetailReq: DiaryDetailReq) => {
  const { data, execute } = useGet<DiaryDetailRes>('diary/detail', diaryDetailReq)
  await execute()
  return data!
}
