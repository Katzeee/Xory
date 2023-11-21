import { defineStore } from 'pinia'
import { ref, reactive } from 'vue'
import { login } from '@/api/login'
import type { LoginReq, DiaryTag } from '@/api/login'

export interface UserInfo {
  email: string
  username: string
  phone: string
}

export const useUserStore = defineStore('user', {
  state: () => ({
    token: String(),
    userInfo: {
      uid: Number(),
      email: String(),
      username: String(),
      phone: String(),
      tags: Array<DiaryTag>()
    }
  }),
  getters: {},
  actions: {
    async login(loginReq: LoginReq) {
      await login(loginReq).then((data) => {
        this.token = data.value!.token
        this.userInfo.uid = data.value!.uid
        this.userInfo.tags = data.value!.tags
        console.log(this.userInfo)
      })
      return this.token
    }
  }
})
