import { createRouter, createWebHistory } from 'vue-router';
import Home from '../pages/Home.vue';
import OpenAPIUploader from '../pages/OpenAPIUploader.vue';

const routes = [
  { path: '/', name: 'Home', component: Home },
  { path: '/openapi-uploader', name: 'OpenAPIUploader', component: OpenAPIUploader },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

export default router;
