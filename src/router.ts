import { createRouter, createWebHashHistory } from 'vue-router';

import Main from './view/Main.vue';
import CreateSolution from './view/CreateSolution.vue';

const routes = [
  { path: '/', name: 'main', component: Main },
  { path: '/create_solution', component: CreateSolution },
];

const router = createRouter({
  history: createWebHashHistory(),
  routes,
});
export default router;
