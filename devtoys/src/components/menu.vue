<script setup lang="ts">
import {
  MenuFoldOutlined,
  MenuUnfoldOutlined,
  SettingOutlined,
} from "@vicons/antd";
import { Search, HomeOutline, FingerPrint } from "@vicons/ionicons5";
import { Engine24Regular, Password24Regular } from "@vicons/fluent";
import { h, ref } from "vue";
import { MenuOption, NIcon, NMenu } from "naive-ui";
import type { Component } from "vue";
import IconFont from "./iconfont.vue";
import PluginLineIcon from "@iconify-vue/clarity/plugin-line";
import router from "../router";

const collapsed = defineModel<boolean>("collapsed", { default: false });

const activeKey = ref<string | null>(null);

const menuOptions = ref<MenuOption[]>([
  {
    label: "所有工具",
    key: "all-tools",
    icon: renderIcon(HomeOutline),
  },
  {
    label: "生成器",
    key: "generators",
    icon: renderIcon(Engine24Regular),
    children: [
      {
        label: "密码",
        key: "password-generator",
        icon: renderIcon(Password24Regular),
      },
      {
        label: "UUID",
        key: "uuid-generator",
        icon: renderIconFont("uuID"),
      },
      {
        label: "哈希/校验和",
        key: "hash-checksum-generator",
        icon: renderIcon(FingerPrint),
      },
    ],
  },
]);

function renderIcon(icon: Component) {
  return () => h(NIcon, null, { default: () => h(icon) });
}

function renderIconFont(name: string) {
  return () => h(IconFont, { name });
}

function handleSetting() {
  router.push({ name: "setting" });
}

function handleClickSearch() {
  console.log("handleClickSearch");
  collapsed.value = false;
  console.log("collapsed", collapsed.value);
}

function handleCollapse() {
  collapsed.value = !collapsed.value;
}
</script>

<template>
  <div class="menu_layout">
    <!-- 顶部固定 -->
    <div class="menu_header">
      <n-button
        text
        secondary
        size="large"
        type="default"
        :bordered="false"
        @click="handleCollapse">
        <template #icon>
          <n-icon v-if="collapsed">
            <MenuUnfoldOutlined />
          </n-icon>
          <n-icon v-else>
            <MenuFoldOutlined />
          </n-icon>
        </template>
      </n-button>
    </div>
    <div class="menu_search">
      <div v-if="!collapsed" style="width: 100%">
        <n-input
          placeholder="输入以搜索工具"
          size="medium"
          type="text"
          clearable>
          <template #suffix>
            <n-icon>
              <Search />
            </n-icon>
          </template>
        </n-input>
      </div>
      <div v-else>
        <n-tooltip>
          <template #trigger>
            <n-button
              text
              type="default"
              size="medium"
              @click="handleClickSearch">
              <template #icon>
                <Search />
              </template>
            </n-button>
          </template>
          <span>输入以搜索工具</span>
        </n-tooltip>
      </div>
    </div>
    <div class="menu_content">
      <n-scrollbar>
        <n-menu
          v-model:value="activeKey"
          :options="menuOptions"
          :collapsed="collapsed"
          :icon-size="16"
          :indent="20" />
      </n-scrollbar>
    </div>
    <div class="menu_footer">
      <n-button text type="default" size="medium" @click="handleSetting">
        <template #icon>
          <SettingOutlined />
        </template>
        <span v-if="!collapsed">设置</span>
      </n-button>
      <n-button text type="default" size="medium">
        <template #icon>
          <PluginLineIcon />
        </template>
        <span v-if="!collapsed">管理扩展</span>
      </n-button>
    </div>
  </div>
</template>

<style scoped>
.menu_layout {
  display: flex;
  flex-direction: column;
  height: 100vh;
  width: 100%;
}

.menu_header {
  flex-shrink: 0;
  padding: 10px;
  display: flex;
  align-items: start;
}

.menu_search {
  flex-shrink: 0;
  padding: 10px;
  display: flex;
  align-items: start;
}

.menu_content {
  flex: 1;
  overflow: hidden;
}

.menu_footer {
  flex-shrink: 0;
  border-top: 1px solid #e0e0e0;
  padding: 10px;
  display: flex;
  justify-content: space-between;
  align-items: start;
  flex-direction: column;
  gap: 10px;
}
</style>
