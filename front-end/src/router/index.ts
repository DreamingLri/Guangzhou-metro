import { createRouter, createWebHistory } from 'vue-router'

const routes = [
    {
        path: '/',
        name: 'main',
        component: () => import('../components/Main.vue')
    },
    {
        path: '/:catchAll(.*)',
        name: 'NotFound',
        title: '404 NotFound',
        component: ()=> import('../components/404.vue')
    }
]

const router = createRouter({
    history: createWebHistory(),
    routes
})

export default router