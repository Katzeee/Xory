import { defineStore } from 'pinia'
import { ref, reactive } from 'vue'
import { login } from '@/api/login'
import type { LoginReq } from '@/api/login'

export interface UserInfo {
  email: string
  username: string
  phone: string
}

export const useUserStore = defineStore('user', {
  state: () => ({
    token: '',
    userInfo: {
      email: '',
      username: '',
      phone: ''
    }
  }),
  actions: {
    async login(loginReq: LoginReq) {
      login(loginReq).then((data) => {
        this.token = data.value!
      })
    }
  }
})
