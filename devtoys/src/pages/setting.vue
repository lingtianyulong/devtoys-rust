<script lang="ts" setup>
import SettingItem from "../components/settingitem.vue";
import type { SelectOption } from "naive-ui";
import type { Component } from "vue";
import { NIcon } from "naive-ui";
import { h } from "vue";
import { Language } from "@vicons/ionicons5";
import {
  ColorPaletteOutline,
  InformationCircleOutline,
} from "@vicons/ionicons5";
import { storeToRefs } from "pinia";
import { useThemeStore } from "../store/theme";

const themeStore = useThemeStore();
const { dark } = storeToRefs(themeStore);

type SettingItem = {
  key: string;
  icon: Component;
  title: string;
  description: string;
  type?: "select" | "switch" | string;
  options?: SelectOption[];
  switchCheckedContent?: string;
  switchUncheckedContent?: string;
  onSwitchChange?: (value: boolean) => void;
};

function renderIcon(icon: Component) {
  return () => h(NIcon, { size: 24 }, { default: () => h(icon) });
}

const appearanceItems: SettingItem[] = [
  {
    key: "language",
    icon: renderIcon(Language),
    title: "界面语言",
    description: "更改语言后, 需要重启应用才能生效.",
    type: "select",
    options: [
      {
        label: "中文(简体)",
        value: "zh-CN",
      },
      {
        label: "中文(繁体)",
        value: "zh-TW",
      },
      {
        label: "English",
        value: "en-US",
      },
    ],
  },
  {
    key: "theme",
    icon: renderIcon(ColorPaletteOutline),
    title: "应用主题",
    description: "选择要使用的主题.",
    type: "switch",
    switchCheckedContent: "深色",
    switchUncheckedContent: "浅色",
  },
];

const aboutItems: SettingItem[] = [
  {
    key: "version",
    icon: renderIcon(InformationCircleOutline),
    title: "DevToys Rust",
    description: "当前版本 v0.1.0",
  },
];

function handleSwitchChange(item: SettingItem, value: boolean) {
  if (item.key === "theme") {
    themeStore.setDark(value);
  }
}
</script>

<template>
  <n-card class="main_card">
    <div class="title">设置</div>
    <div class="group_title">外观</div>
    <n-space vertical :size="8">
      <SettingItem
        v-for="item in appearanceItems"
        :key="item.key"
        :icon="item.icon"
        :title="item.title"
        :description="item.description"
        :type="item.type"
        :options="item.options"
        :switchCheckedContent="item.switchCheckedContent"
        :switchUncheckedContent="item.switchUncheckedContent"
        :switchValue="item.key === 'theme' ? dark : undefined"
        :onSwitchChange="(value) => handleSwitchChange(item, value)" />
    </n-space>

    <div class="group_title" style="margin-top: 20px">关于</div>
    <n-space vertical :size="8">
      <SettingItem
        v-for="item in aboutItems"
        :key="item.key"
        :icon="item.icon"
        :title="item.title"
        :description="item.description" />
    </n-space>
  </n-card>
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
</style>
