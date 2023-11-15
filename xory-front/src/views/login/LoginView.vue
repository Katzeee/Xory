<template>
  <div>
    <el-form
      :model="loginForm"
      ref="loginFormRef"
      :rules="rules"
      label-width="80px"
      :inline="false"
      :size="size"
    >
      <el-form-item :label="t('email/phone')" prop="accountIdentifier">
        <el-input v-model="loginForm.accountIdentifier"></el-input>
      </el-form-item>
      <el-form-item :label="t('password')" :size="size" prop="password">
        <el-input v-model="loginForm.password" placeholder="" :size="size" clearable></el-input>
      </el-form-item>

      <el-form-item>
        <el-button type="primary" @click="onSubmit">{{ t('login') }}</el-button>
        <!-- <el-button>{{ t('cancel') }}</el-button> -->
      </el-form-item>
    </el-form>
  </div>
</template>

<script setup lang="ts">
import { reactive, ref } from 'vue'
import type { FormInstance, FormRules } from 'element-plus'
import { useI18n } from 'vue-i18n'
import type { MessageSchema } from '@/i18n'
const { t } = useI18n<{ message: MessageSchema }>({ useScope: 'global' })
const loginFormRef = ref<FormInstance>()
const size = ref('default')

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

const onSubmit = async () => {
  await loginFormRef.value
    ?.validate()
    .then(() => console.log(loginForm.accountIdentifier, loginForm.password))
    .catch(() => console.log('error'))
}
</script>
