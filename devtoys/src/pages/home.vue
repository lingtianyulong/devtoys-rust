<script setup lang="ts">
import { useThemeStore } from "../store/theme";

const themeStore = useThemeStore();
</script>

<template>
  <div class="home">
    <div class="home-content">
      <n-switch :value="themeStore.dark" @update:value="themeStore.setDark">
        暗黑模式
      </n-switch>
    </div>
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
