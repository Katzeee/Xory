import { useGet } from '@/request'

export interface DiaryListReq {
  uid: number
  page_number: number
  page_size: number
  keywords: string
  category: string
}

export interface DiaryListRes {
  title?: string | undefined
  content?: string
  id?: number
  category?: number
  date?: Date | string
}

export const diaryList = async (diaryListReq: DiaryListReq) => {
  const { data, execute } = useGet<DiaryListRes[]>('diary/list', diaryListReq)
  await execute()
  return data!
}

export interface DiaryDetailReq {
  id: number
}

export interface DiaryDetailRes {
  id?: number
  date?: Date
  title?: string
  content?: string
  temperature?: number
  weather?: string
  category?: number
  date_create?: Date
  date_modify?: Date
  uid?: number
  longitude?: number
  latitude?: number
}

export const diaryDetail = async (diaryDetailReq: DiaryDetailReq) => {
  const { data, execute } = useGet<DiaryDetailRes>('diary/detail', diaryDetailReq)
  await execute()
  return data!
}
