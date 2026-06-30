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
import { h, ref, type Component, onMounted, onUnmounted } from "vue";
import { BorderlessTableOutlined } from "@vicons/antd";
import IconFont from "../../components/iconfont.vue";
import {
  TextCaseUppercase24Regular,
  BookInformation24Regular,
} from "@vicons/fluent";

import { Refresh } from "@vicons/ionicons5";
import { ContentCopyRound } from "@vicons/material";
import { Save20Regular } from "@vicons/fluent";
import { invoke } from "@tauri-apps/api/core";
import { writeText as writeTextToClipboard } from "@tauri-apps/plugin-clipboard-manager";
import { save as saveFile } from "@tauri-apps/plugin-dialog";
import { writeTextFile, BaseDirectory } from "@tauri-apps/plugin-fs";

type SettingSelectValue = string | number | null;

type SettingItem = {
  key: string;
  icon: Component;
  title: string;
  description: string;
  type?: "select" | "switch" | string;
  options?: SelectOption[];
  switchCheckedContent?: string;
  switchUncheckedContent?: string;
  onSelectChange?: (value: SettingSelectValue) => void;
  onSwitchChange?: (value: boolean) => void;
};

type UUIDParams = {
  hyphen: boolean;
  uppercase: boolean;
  version: string | null;
  count: number;
};

const uuidList = ref<string[]>([]);
const uuidText = ref<string>("");

const uuidParams = ref<UUIDParams>({
  hyphen: true,
  uppercase: false,
  version: "v4",
  count: 1,
});

onMounted(async () => {
  console.log("onMounted");
  try {
    await invoke<string>("generate_uuid", {
      params: JSON.stringify(uuidParams.value),
    }).then((uuids) => {
      if (uuids) {
        uuidList.value = JSON.parse(uuids) as string[];
        uuidText.value = uuidList.value.join("\n");
      }
    });
  } catch (error) {
    console.error(error);
  }
});

onUnmounted(() => {
  // window.removeEventListener("message", handleMessage);
});

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
        label: "v4",
        value: "v4",
      },
      {
        label: "v7",
        value: "v7",
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

function generateUUID() {
  try {
    invoke<string>("generate_uuid", {
      params: JSON.stringify(uuidParams.value),
    }).then((uuids) => {
      if (uuids) {
        uuidList.value = JSON.parse(uuids) as string[];
        uuidText.value = uuidList.value.join("\n");
      }
    });
  } catch (error) {
    console.error(error);
  }
}
function handleSwitchChange(item: SettingItem, value: boolean) {
  if (item.key === "hyphen") {
    uuidParams.value.hyphen = value;
  }
  if (item.key === "uppercase") {
    uuidParams.value.uppercase = value;
  }
  generateUUID();
}

function handleNumberChange(value: number | null) {
  uuidParams.value.count = value ?? 1;
  generateUUID();
}

function handleSelectChange(item: SettingItem, value: SettingSelectValue) {
  if (item.key === "version") {
    uuidParams.value.version = value === null ? null : String(value);
  }
  generateUUID();
}

function createSwitchHandler(item: SettingItem) {
  return (value: boolean) => handleSwitchChange(item, value);
}

function createSelectHandler(item: SettingItem) {
  return (value: SettingSelectValue) => handleSelectChange(item, value);
}

function handleRefresh() {
  generateUUID();
}

async function handleCopy() {
  await writeTextToClipboard(uuidText.value);
}

async function handleSave() {
  await saveFile({
    title: "保存 UUID",
    filters: [
      { name: "UUID Files", extensions: ["txt"] },
      { name: "All Files", extensions: ["*"] },
    ],
  }).then(async (result) => {
    if (result) {
      console.log("result", result);
      console.log("uuidText", uuidText.value);
      await writeTextFile(result, uuidText.value, {
        append: false,
        create: true,
        baseDir: BaseDirectory.AppConfig,
      });
    }
  });
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
            ? uuidParams.hyphen
            : item.key === 'uppercase'
              ? uuidParams.uppercase
              : undefined
        "
        :selectValue="item.key === 'version' ? uuidParams.version : undefined"
        :switchCheckedContent="item.switchCheckedContent"
        :switchUncheckedContent="item.switchUncheckedContent"
        :onSelectChange="createSelectHandler(item)"
        :onSwitchChange="createSwitchHandler(item)"
        :onNumberChange="handleNumberChange" />
    </n-space>

    <n-flex justify="space-between" align="center" style="margin-top: 20px">
      <div>UUID(s)</div>
      <div>
        <n-button style="margin-right: 10px" @click="handleRefresh">
          <template #icon>
            <n-icon>
              <Refresh />
            </n-icon>
          </template>
          刷新
        </n-button>
        <n-button style="margin-right: 10px" @click="handleCopy">
          <template #icon>
            <n-icon>
              <ContentCopyRound />
            </n-icon>
          </template>
          复制
        </n-button>
        <n-button @click="handleSave">
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
      <n-input
        :value="uuidText"
        type="textarea"
        :rows="15"
        readonly
        :resizable="false"
        placeholder="" />
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
