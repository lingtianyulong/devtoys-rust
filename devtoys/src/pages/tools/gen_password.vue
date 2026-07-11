<script lang="ts" setup></script>

<template>
  <el-card class="main_card">
    <div class="title">密码生成器</div>
    <div class="group_title">参数配置</div>
    <el-space direction="vertical" :size="8" fill style="width: 98%">
      <el-card class="item_card">
        <div>长度</div>
      </el-card>
      <el-card class="item_card">
        <div>包含特殊字符</div>
      </el-card>
    </el-space>
  </el-card>
</template>

<style scoped>
.main_card {
  display: flex;
  flex-direction: column;
  height: 100vh;
  width: 100%;
  overflow-y: auto;
}

.title {
  width: 100%;
  font-size: 24px;
  font-weight: bold;
  text-align: center;
  margin-bottom: 20px;
  color: #333;
  display: flex;
  justify-content: start;
  align-items: center;
}

.group_title {
  font-size: 18px;
  text-align: center;
  margin-bottom: 10px;
  color: #333;
  display: flex;
  justify-content: start;
  align-items: center;
}

.item_card {
  display: flex;
  flex-direction: row;
  align-items: center;
  justify-content: start;
  padding: 10px;
  width: 100%;
  height: 100%;
}

.item_card :deep(.el-card__body) {
  display: flex;
  flex-direction: row;
  align-items: center;
  gap: 12px;
  width: 100%;
  box-sizing: border-box;
}
</style>
