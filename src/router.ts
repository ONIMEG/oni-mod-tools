import { createRouter, createWebHashHistory, Router } from 'vue-router';

import Main from './view/Main.vue';
import CreateSolution from './view/CreateSolution.vue';
import Solutions from './view/Solutions.vue';

const routes = [
  { path: '/', name: 'main', component: Main },
  { path: '/create_solution', component: CreateSolution },
  { path: '/solution', component: Solutions },
];

const router: Router = createRouter({
  history: createWebHashHistory(),
  routes,
});
export default router;
