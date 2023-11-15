import {
  createFetch,
  isObject,
  type MaybeRef,
  type UseFetchOptions,
  type UseFetchReturn
} from '@vueuse/core'
import { ElMessage } from 'element-plus'
import { useI18n } from 'vue-i18n'
import type { MessageSchema } from '@/i18n'

const { t } = useI18n<{ message: MessageSchema }>({ useScope: 'global' })
const baseUrl = import.meta.env.VITE_API_BASE_URL
const useToken = () => {
  const isExpiredSoon = true
  const token = '123'
  return { isExpiredSoon, token }
}
const RequestTimeout = 1000 * 60 * 10
const ErrorFlag = '____________'

export const useRequest = createFetch({
  baseUrl,
  options: {
    immediate: false,
    timeout: RequestTimeout,
    beforeFetch({ options }) {
      const { token } = useToken()
      options.headers = Object.assign(options.headers || {}, {
        Authorization: token
      })
      return { options }
    },
    afterFetch: function ({ data, response }) {
      const { isExpiredSoon } = useToken()
      const status = data.code || 200
      if (status === 200) {
        data = data.data || {}
      } else if (status === 500) {
        ElMessage.error(data.msg)
        data = ErrorFlag
      } else if (isExpiredSoon) {
        // 最后验证本地token效期,快过期时,刷新token
        // useUserStore().freshToken()
      }
      return { data, response }
    },
    onFetchError({ response, error }) {
      // if (response?.status === 401) {
      //   ElMessage.warning(t('commonTip.loginExpired'))
      //   useUserStore().frontEndLogout()
      //   setTimeout(() => {
      //     router.push('/login')
      //   }, 500)
      // }
      return { error }
    }
  }
})
