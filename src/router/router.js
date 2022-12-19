import { createRouter, createWebHistory } from "vue-router";

import Main from "../views/Main.vue";
import About from "../views/About.vue";
import Config from "../views/Config.vue";

const routes = [
  { path: "/", component: Main },
  { path: "/about", component: About },
  { path: "/config", component: Config },
];

const router = createRouter({
  history: createWebHistory(),
  routes,
});

export default router;
