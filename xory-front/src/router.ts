import { createRouter, createWebHistory } from 'vue-router'
import DiaryListView from '@/views/diary/DiaryListView.vue'
import LoginView from '@/views/login/LoginView.vue'
import AboutView from '@/views/AboutView.vue'
import { useUserStore } from '@/stores/user'

const routes = [
  {
    path: '/diary',
    // component: AboutView,
    children: [{ path: 'list', component: DiaryListView }]
  },
  {
    path: '/about',
    name: 'About',
    // route level code-splitting
    // this generates a separate chunk (About.[hash].js) for this route
    // which is lazy-loaded when the route is visited.
    component: AboutView
  },
  {
    path: '/login',
    name: 'Login',
    component: LoginView
  }
]

const routerInner = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: routes
})

export const router = () => {
  const userStore = useUserStore()

  routerInner.beforeEach((to, from, next) => {
    switch (to.name) {
      case 'Login':
        next()
        break
      default:
        if (userStore.token !== '') {
          next()
        } else {
          next('/login')
        }
        break
    }
  })
  return routerInner
}
