<!--
 * @Author: huzhiqiang
 * @Date: 2026-06-18 15:00:00
 * @LastEditors: huzhiqiang
 * @LastEditTime: 2026-06-18 15:00:00
 * @FilePath: \devtoys\src\pages\tools\uuid.vue
 * @Description: A page for UUID tool.
-->

<script lang="ts" setup>
import SettingItem from "../../components/settingitem.vue";
import { NIcon, type SelectOption } from "naive-ui";
import { h, ref, type Component } from "vue";
import { BorderlessTableOutlined } from "@vicons/antd";
import IconFont from "../../components/iconfont.vue";
import {
  TextCaseUppercase24Regular,
  BookInformation24Regular,
} from "@vicons/fluent";

import { Refresh } from "@vicons/ionicons5";
import { ContentCopyRound } from "@vicons/material";
import { Save20Regular } from "@vicons/fluent";

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

function renderIconFont(name: string) {
  return () => h(IconFont, { name });
}

const appearanceItems: SettingItem[] = [
  {
    key: "hyphen",
    icon: renderIconFont("icon-point-hyphen"),
    title: "连字符",
    description: "选择是否包含连字符.",
    type: "switch",
    switchCheckedContent: "开启",
    switchUncheckedContent: "关闭",
  },
  {
    key: "uppercase",
    icon: renderIcon(TextCaseUppercase24Regular),
    title: "大写字母",
    description: "选择是否使用大写字母.",
    type: "switch",
    switchCheckedContent: "开启",
    switchUncheckedContent: "关闭",
  },
  {
    key: "version",
    icon: renderIcon(BookInformation24Regular),
    title: "UUID 版本",
    description: "选择要生成的 UUID 版本.",
    type: "select",
    options: [
      {
        label: "v1",
        value: "v1",
      },
      {
        label: "v3",
        value: "v3",
      },
      {
        label: "v4",
        value: "v4",
      },
      {
        label: "v5",
        value: "v5",
      },
      {
        label: "v6",
        value: "v6",
      },
      {
        label: "v7",
        value: "v7",
      },
      {
        label: "v8",
        value: "v8",
      },
    ],
  },
  {
    key: "gen_count",
    icon: renderIcon(BorderlessTableOutlined),
    title: "生成数量",
    description: "选择要生成的 UUID 数量.",
    type: "number",
  },
];

const hyphen = ref(false);
const uppercase = ref(false);

function handleSwitchChange(item: SettingItem, value: boolean) {
  if (item.key === "hyphen") {
    hyphen.value = value;
  }
  if (item.key === "uppercase") {
    uppercase.value = value;
  }
}

function handleNumberChange(value: number | null) {
  console.log(value);
}

function createSwitchHandler(item: SettingItem) {
  return (value: boolean) => handleSwitchChange(item, value);
}
</script>

<template>
  <n-card class="main_card">
    <div class="title">UUID 生成器</div>
    <div class="group_title">参数配置</div>
    <n-space vertical :size="8">
      <SettingItem
        v-for="item in appearanceItems"
        :key="item.key"
        :icon="item.icon"
        :title="item.title"
        :description="item.description"
        :type="item.type"
        :options="item.options"
        :switchValue="
          item.key === 'hyphen'
            ? hyphen
            : item.key === 'uppercase'
              ? uppercase
              : undefined
        "
        :switchCheckedContent="item.switchCheckedContent"
        :switchUncheckedContent="item.switchUncheckedContent"
        :onSwitchChange="createSwitchHandler(item)"
        :onNumberChange="handleNumberChange" />
    </n-space>

    <n-flex justify="space-between" align="center" style="margin-top: 20px">
      <div>UUID(s)</div>
      <div>
        <n-button style="margin-right: 10px">
          <template #icon>
            <n-icon>
              <Refresh />
            </n-icon>
          </template>
          刷新
        </n-button>
        <n-button style="margin-right: 10px">
          <template #icon>
            <n-icon>
              <ContentCopyRound />
            </n-icon>
          </template>
          复制
        </n-button>
        <n-button>
          <template #icon>
            <n-icon>
              <Save20Regular />
            </n-icon>
          </template>
          保存
        </n-button>
      </div>
    </n-flex>
    <div style="margin-top: 20px">
      <n-input type="textarea" :rows="15" :resizable="false" />
    </div>
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
