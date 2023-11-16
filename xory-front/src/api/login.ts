import { useGet } from '@/request'
import type { Ref } from 'vue'

export interface LoginReq {
  email: string
  password: string
}

export interface LoginRes {
  token: string
  uid: number
}

export const login = async (loginReq: LoginReq) => {
  const { data, execute } = useGet<LoginRes>('user/login', loginReq)
  await execute()
  return data!
}
