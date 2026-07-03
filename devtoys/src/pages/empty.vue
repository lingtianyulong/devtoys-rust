<script lang="ts" setup>
import { ElEmpty } from "element-plus";
</script>
<template>
  <el-empty
    class="empty-page"
    :image-size="80"
    description="功能暂未实现, 敬请期待...">
  </el-empty>
</template>

<style scoped>
.empty-page {
  display: flex;
  justify-content: center;
  align-items: center;
  min-height: 100vh;
  width: 100%;
}

.empty-page :deep(.el-empty__image) {
  display: flex;
  justify-content: center;
  align-items: center;
  width: 80px;
  height: 80px;
}

.empty-page :deep(.icon) {
  display: block;
  width: 2em;
  height: 2em;
  font-size: inherit;
}
</style>
