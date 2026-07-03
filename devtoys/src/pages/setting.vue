<script lang="ts" setup>
import { ElCard, ElIcon, ElSpace } from "element-plus";

import { Language } from "@vicons/ionicons5";
import { ColorPaletteOutline } from "@vicons/ionicons5";
import { storeToRefs } from "pinia";
import { useThemeStore } from "../store/theme";
import { InfoFilled } from "@element-plus/icons-vue";

const themeStore = useThemeStore();
const { dark } = storeToRefs(themeStore);

type SelectOption = {
  label: string;
  value: string | number;
};

const languageOptions: SelectOption[] = [
  {
    label: "中文(简体)",
    value: "zh-CN",
  },
  {
    label: "English",
    value: "en-US",
  },
];

function handleThemeChange(value: boolean) {
  themeStore.setDark(value);
}
</script>

<template>
  <el-card class="main_card">
    <div class="title">设置</div>
    <div class="group_title">外观</div>
    <el-space direction="vertical" :size="8" fill style="width: 95%">
      <el-card class="item_card">
        <div class="left">
          <el-icon :size="24">
            <component :is="Language" />
          </el-icon>
        </div>
        <div class="center">
          <div class="title">界面语言</div>
          <div class="description">更改语言后, 需要重启应用才能生效.</div>
        </div>
        <div class="right">
          <el-select :options="languageOptions" />
        </div>
      </el-card>
      <el-card class="item_card">
        <div class="left">
          <el-icon :size="24">
            <component :is="ColorPaletteOutline" />
          </el-icon>
        </div>
        <div class="center">
          <div class="title">应用主题</div>
          <div class="description">选择要使用的主题.</div>
        </div>
        <div class="right">
          <el-switch
            :model-value="dark"
            :active-value="true"
            :inactive-value="false"
            active-text="深色"
            inactive-text="浅色"
            @update:model-value="handleThemeChange" />
        </div>
      </el-card>
    </el-space>

    <div class="group_title" style="margin-top: 20px">关于</div>
    <el-space direction="vertical" :size="8" fill style="width: 95%">
      <el-card class="item_card">
        <div class="left">
          <el-icon :size="24">
            <InfoFilled />
          </el-icon>
        </div>
        <div class="center">
          <div class="title">DevToys Rust</div>
          <div class="description">当前版本 v0.1.0</div>
        </div>
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
  padding: 20px;
  overflow: hidden;
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

.settingitem_card {
  border-radius: 5px;
}

.setting-row {
  width: 100%;
  flex-direction: row;
  display: flex;
  align-items: center;
}

.left {
  width: 48px;
  display: flex;
  justify-content: start;
  align-items: center;
}

.center {
  flex: 1;
  display: flex;
  flex-direction: column;
}

.title {
  font-size: 16px;
  font-weight: bold;
  color: var(--el-text-color-primary);
}

.description {
  font-size: 14px;
  color: var(--el-text-color-regular);
}

.right {
  width: 180px;
  display: flex;
  justify-content: flex-end;
  align-items: center;
}

.title {
  width: 100%;
  font-size: 24px;
  font-weight: bold;
  text-align: center;
  margin-bottom: 20px;
  color: var(--el-text-color-primary);
  display: flex;
  justify-content: start;
  align-items: center;
}

.group_title {
  font-size: 18px;
  text-align: center;
  margin-bottom: 10px;
  color: var(--el-text-color-primary);
  display: flex;
  justify-content: start;
  align-items: center;
}
</style>
