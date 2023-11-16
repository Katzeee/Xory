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

export const listDiaries = async (diaryListReq: DiaryListReq) => {
  const { data, execute } = useGet<DiaryListRes[]>('diary/list', diaryListReq)
  await execute()
  return data!
}
