import { createApp } from 'vue'
// import './style.css'
import App from './App.vue'
import router from './router'
import Antd from 'ant-design-vue';
import 'ant-design-vue/dist/reset.css';
import ElementPlus from 'element-plus'
import 'element-plus/dist/index.css'

createApp(App)
    .use(router)
    .use(Antd)
    .use(ElementPlus)
    .mount('#app')
