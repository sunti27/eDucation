import { createRouter, createWebHistory } from "vue-router";
import Overview from "@/views/Overview.vue";
import Classroom from "@/views/Classroom.vue";

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: "/overview",
      name: "overview",
      component: Overview,
    },
    {
      path: "/classroom",
      name: "classroom",
      component: Classroom,
    },
  ],
});

export default router;
