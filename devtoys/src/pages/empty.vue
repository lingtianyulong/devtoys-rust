<script lang="ts" setup>
import IconFont from "../components/iconfont.vue";
import { h } from "vue";

const renderIcon = (name: string) => {
  return () => h(IconFont, { name });
};
</script>
<template>
  <n-empty
    class="empty-page"
    size="huge"
    description="功能暂未实现, 敬请期待...">
    <template #icon>
      <component :is="renderIcon('kongyemian')" />
    </template>
  </n-empty>
</template>

<style scoped>
.empty-page {
  display: flex;
  justify-content: center;
  align-items: center;
  min-height: 100vh;
  width: 100%;
}

.empty-page :deep(.n-empty__icon) {
  display: flex;
  justify-content: center;
  align-items: center;
}

.empty-page :deep(.icon) {
  display: block;
  width: 2em;
  height: 2em;
  font-size: inherit;
}
</style>
