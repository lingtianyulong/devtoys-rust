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
        {
          path: "/empty",
          name: "empty",
          component: () => import("../pages/empty.vue"),
        },
        {
          path: "/tools/uuid",
          name: "tools-uuid",
          component: () => import("../pages/tools/uuid.vue"),
        },
        {
          path: "/tools/camera-selection",
          name: "tools-camera-selection",
          component: () => import("../pages/tools/camera-selection.vue"),
        },
        {
          path: "/tools/gen_password",
          name: "tools-gen-password",
          component: () => import("../pages/tools/gen_password.vue"),
        },
      ],
    },
  ],
});

export default router;
