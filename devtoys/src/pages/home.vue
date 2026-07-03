<script setup lang="ts">
import Menu from "../components/menu.vue";
import { ElAside, ElMain, ElContainer } from "element-plus";
import { ref, watch } from "vue";

const SPLIT_MIN = 0.04;
const SPLIT_DEFAULT = 0.2;

const collapsed = ref(false);
const splitSize = ref(SPLIT_DEFAULT);

watch(collapsed, (isCollapsed) => {
  splitSize.value = isCollapsed ? SPLIT_MIN : SPLIT_DEFAULT;
});
</script>

<template>
  <el-container class="home">
    <el-aside class="home-sidebar">
      <Menu v-model:collapsed="collapsed" />
    </el-aside>
    <el-main class="home-content">
      <router-view />
    </el-main>
  </el-container>
</template>

<style scoped>
:global(html),
:global(body),
:global(#app) {
  width: 100%;
  height: 100%;
  overflow: hidden;
  scrollbar-width: none;
  -ms-overflow-style: none;
}

:global(html::-webkit-scrollbar),
:global(body::-webkit-scrollbar),
:global(#app::-webkit-scrollbar) {
  display: none;
}

.home {
  display: flex;
  height: 100vh;
  width: 100%;
  overflow: hidden;
  scrollbar-width: none;
  -ms-overflow-style: none;
}

.home::-webkit-scrollbar {
  display: none;
}

.home :deep(.el-main) {
  overflow: hidden;
  scrollbar-width: none;
  -ms-overflow-style: none;
}

.home :deep(.el-main)::-webkit-scrollbar {
  display: none;
}

.home-sidebar {
  flex: 0 0 20%;
  min-width: 48px;
  max-width: 25%;
  height: 100%;
  border-right: 1px solid #e5e5e5;
}

.home-content {
  flex: 1;
  overflow: hidden;
  padding: 0;
}
</style>
