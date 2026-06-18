import { createRouter, createWebHistory } from "vue-router";

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: "/",
      redirect: "/home",
    },
    {
      path: "/home",
      name: "home",
      component: () => import("../pages/home.vue"),
      children: [
        {
          path: "/setting",
          name: "setting",
          component: () => import("../pages/setting.vue"),
        },
      ],
    },
  ],
});

export default router;
