import { useGet } from '@/request'

export interface DiaryListReq {
  uid: number
  page_number: number
  page_size: number
  keywords: string
  category: string
}

export interface DiaryListRes {
  title: string
  content: string
  id: number
  category: number
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
  id: number
  date: DateTime
  title: string
  content?: string
  temperature?: number
  weather?: string
  category: number
  date_create: DateTime
  date_modify: DateTime
  uid: number
  longitude: number
  latitude: number
}

export const diaryDetail = async (diaryDetailReq: DiaryDetailReq) => {
  const { data, execute } = useGet<DiaryDetailRes>('diary/detail', diaryDetailReq)
  await execute()
  return data!
}
