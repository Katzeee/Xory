import { useGet } from '@/request'

export interface LoginReq {
  email: string
  password: string
}

export const login = async (loginReq: LoginReq) => {
  const { data, execute } = useGet('user/login', loginReq)
  await execute()
  return data
}
