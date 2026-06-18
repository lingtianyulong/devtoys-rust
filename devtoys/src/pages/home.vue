<script setup lang="ts">
import Menu from "../components/menu.vue";
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
  <div class="home">
    <n-split
      v-model:size="splitSize"
      direction="horizontal"
      style="height: 100vh"
      :max="0.25"
      :min="SPLIT_MIN">
      <template #1> <Menu v-model:collapsed="collapsed" /> </template>
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
