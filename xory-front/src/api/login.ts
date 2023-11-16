import { useGet } from '@/request'
import type { Ref } from 'vue'

export interface LoginReq {
  email: string
  password: string
}

export const login = async (loginReq: LoginReq) => {
  const { data, execute } = useGet<string>('user/login', loginReq)
  await execute()
  return data
}
