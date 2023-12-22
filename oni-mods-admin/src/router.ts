import { createRouter, createWebHashHistory, Router } from 'vue-router';

import Main from './view/Main.vue';
import CreateSolution from './view/CreateSolution.vue';
import Solutions from './view/Solutions.vue';
import CsProject from './view/CsProject.vue';
import Setting from './view/Setting.vue';

const routes = [
  { path: '/', name: 'main', component: Main },
  { path: '/create_solution', component: CreateSolution },
  { path: '/solution', component: Solutions },
  { path: '/csproj', component: CsProject },
  { path: '/setting', component: Setting },
];

const router: Router = createRouter({
  history: createWebHashHistory(),
  routes,
});
export default router;
