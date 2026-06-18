<script setup lang="ts">
import Menu from "../components/menu.vue";
// import { useThemeStore } from "../store/theme";

// const themeStore = useThemeStore();
</script>

<template>
  <div class="home">
    <n-split
      direction="horizontal"
      style="height: 100vh"
      :default-size="0.2"
      :max="0.25"
      :min="0.04">
      <template #1> <Menu /> </template>
      <template #2>
        <n-layout-content>
          <router-view />
        </n-layout-content>
      </template>
    </n-split>
  </div>
</template>

<style scoped>
.home {
  display: flex;
  flex-direction: column;
  height: 100%;
}

.home-titlebar {
  flex: 0 0 36px;
}

.home-content {
  flex: 1;
  overflow: auto;
}
</style>
