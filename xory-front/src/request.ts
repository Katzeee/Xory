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
import { computed, unref } from 'vue'
import { type LocationQueryRaw, stringifyQuery } from 'vue-router'

// const { t } = useI18n<{ message: MessageSchema }>({ useScope: 'global' })
const baseUrl = import.meta.env.VITE_BACKEND_API_BASE_URL
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
      } else {
        ElMessage.error(data.message)
        data = ErrorFlag
      }
      // else if (isExpiredSoon) {
      // 最后验证本地token效期,快过期时,刷新token
      // useUserStore().freshToken()
      // }
      return { data, response }
    },
    onFetchError({ response, error }) {
      ElMessage.error('fetch error')
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

export const getQueryUrl = (url: MaybeRef<string>, query?: MaybeRef<unknown>) => {
  return computed(() => {
    const _url = unref(url)
    const _query = unref(query)
    const queryString = isObject(_query) ? stringifyQuery(_query as LocationQueryRaw) : _query || ''
    return `${_url}${queryString ? '?' : ''}${queryString}`
  })
}

export function useGet<T = unknown>(
  url: MaybeRef<string>,
  query?: MaybeRef<unknown>,
  options?: UseFetchOptions
): UseFetchReturn<T> {
  return useRequest<T>(getQueryUrl(url, query), { ...options }).json()
}
