<template>
  <v-container class="column-root-container">
    <div class="login-form">
      <el-form :model="loginForm" ref="loginFormRef" :rules="rules" :inline="false" :size="size">
        <el-form-item prop="accountIdentifier">
          <el-input
            v-model="loginForm.accountIdentifier"
            :placeholder="t('login.placeholder.email/phone')"
            :size="size"
          ></el-input>
        </el-form-item>
        <el-form-item :size="size" prop="password">
          <el-input
            v-model="loginForm.password"
            type="password"
            :placeholder="t('login.placeholder.password')"
            :size="size"
          ></el-input>
        </el-form-item>

        <el-form-item class="button-group">
          <el-button type="primary" @click="onSubmit">{{ t('login.button.login') }}</el-button>
          <el-button @click="onTest">{{ t('login.button.test') }}</el-button>
        </el-form-item>
      </el-form>
    </div>
  </v-container>
</template>

<script setup lang="ts">
import { reactive, ref } from 'vue'
import type { FormInstance, FormRules } from 'element-plus'
import { useI18n } from 'vue-i18n'
import type { MessageSchema } from '@/i18n'
import { useUserStore } from '@/stores/user'
import { useRouter } from 'vue-router'
const router = useRouter()
const { t } = useI18n<{ message: MessageSchema }>({ useScope: 'global' })

const loginFormRef = ref<FormInstance>()
const size = ref('large')

interface LoginForm {
  accountIdentifier: string
  password: string
}
const loginForm = reactive({
  accountIdentifier: '',
  password: ''
})
const rules = reactive<FormRules<LoginForm>>({
  accountIdentifier: [{ required: true, message: t('login.tip.requireId'), trigger: 'blur' }]
})

const userStore = useUserStore()
const onSubmit = async () => {
  await loginFormRef.value
    ?.validate()
    .then(() => {
      userStore
        .login({ email: loginForm.accountIdentifier, password: loginForm.password })
        .then(() => {
          router.push('/diary/list')
        })
    })
    .catch(() => console.log('error'))
}
const onTest = () => {
  loginForm.accountIdentifier = 'ttrumpeter4@wordpress.com'
  loginForm.password = 'test'
  loginFormRef.value!.validate()
}
</script>

<style scoped lang="scss">
.column-root-container {
  justify-content: center;
}
.el-form {
  display: flex;
  flex-direction: column;
  .el-form-item {
    margin-bottom: 22px;
    flex: 1;
  }
  .button-group :deep(.el-form-item__content) {
    display: flex;
    justify-content: space-around;
  }
}
</style>
