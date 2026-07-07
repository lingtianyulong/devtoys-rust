<script setup lang="ts">
import { Search, HomeOutline, EyeSharp } from "@vicons/ionicons5";
import { Engine24Regular, Camera24Regular } from "@vicons/fluent";
import { h, ref } from "vue";
import { ElIcon } from "element-plus";
import type { Component } from "vue";
import IconFont from "./iconfont.vue";
import router from "../router";
import { Setting } from "@element-plus/icons-vue";

const collapsed = defineModel<boolean>("collapsed", { default: false });

const activeKey = ref<string | null>(null);

type MenuItemOption = {
  label: string;
  key: string;
  icon?: Component;
  path?: string;
  children?: MenuItemOption[];
};

const menuOptions = ref<MenuItemOption[]>([
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
        label: "UUID",
        key: "uuid-generator",
        path: "/tools/uuid",
        icon: renderIconFont("bd_uuid"),
      },
    ],
  },
  {
    label: "机器视觉",
    key: "machine-vision",
    icon: renderIcon(EyeSharp),
    children: [
      {
        label: "相机选型",
        key: "camera-selection",
        path: "/tools/camera-selection",
        icon: renderIcon(Camera24Regular),
      },
    ],
  },
]);

function renderIcon(icon: Component) {
  return (props: { size?: number | string; class?: string }) =>
    h(
      ElIcon,
      { size: props.size, class: props.class },
      { default: () => h(icon) },
    );
}

function renderIconFont(name: string) {
  return () => h(IconFont, { name });
}

function handleSetting() {
  router.push({ name: "setting" });
}

function handleOpenPlugin() {
  router.push({ name: "empty" });
}

function handleClickSearch() {
  console.log("handleClickSearch");
  collapsed.value = false;
  console.log("collapsed", collapsed.value);
}

function handleCollapse() {
  collapsed.value = !collapsed.value;
}

function findMenuItem(
  options: MenuItemOption[],
  key: string,
): MenuItemOption | undefined {
  for (const option of options) {
    if (option.key === key) {
      return option;
    }

    if (option.children) {
      const child = findMenuItem(option.children, key);
      if (child) {
        return child;
      }
    }
  }
}

function handleMenuSelect(key: string) {
  activeKey.value = key;
  const item = findMenuItem(menuOptions.value, key);
  if (item?.path) {
    router.push(item.path);
  }
}
</script>

<template>
  <div class="menu_layout">
    <!-- 顶部固定 -->
    <div class="menu_header">
      <el-button text type="default" @click="handleCollapse">
        <template #icon>
          <el-icon :size="20" v-if="collapsed">
            <Expand />
          </el-icon>
          <el-icon :size="20" v-else>
            <Fold />
          </el-icon>
        </template>
      </el-button>
    </div>
    <div class="menu_search">
      <div v-if="!collapsed" style="width: 100%">
        <el-input
          placeholder="输入以搜索工具"
          size="default"
          type="text"
          clearable>
          <template #suffix>
            <el-icon>
              <Search />
            </el-icon>
          </template>
        </el-input>
      </div>
      <div v-else>
        <el-tooltip content="输入以搜索工具">
          <el-button text type="default" @click="handleClickSearch">
            <template #icon>
              <Search :size="20" />
            </template>
          </el-button>
        </el-tooltip>
      </div>
    </div>
    <div class="menu_content">
      <el-scrollbar>
        <el-menu
          :default-active="activeKey ?? ''"
          :collapse="collapsed"
          :collapse-transition="false"
          :indent="20"
          @select="handleMenuSelect"
          style="border: none">
          <template v-for="item in menuOptions" :key="item.key">
            <el-sub-menu v-if="item.children?.length" :index="item.key">
              <template #title>
                <component v-if="item.icon" :is="item.icon" :size="20" />
                <span>{{ item.label }}</span>
              </template>
              <el-menu-item
                v-for="child in item.children"
                :key="child.key"
                :index="child.key">
                <component
                  v-if="child.icon"
                  :is="child.icon"
                  :size="20"
                  style="margin-right: 10px" />
                <span>{{ child.label }}</span>
              </el-menu-item>
            </el-sub-menu>
            <el-menu-item v-else :index="item.key">
              <component v-if="item.icon" :is="item.icon" />
              <span>{{ item.label }}</span>
            </el-menu-item>
          </template>
        </el-menu>
      </el-scrollbar>
    </div>
    <div class="menu_footer">
      <el-button text type="default" size="large" @click="handleSetting">
        <template #icon>
          <el-icon :size="20">
            <Setting />
          </el-icon>
        </template>
        <div v-if="!collapsed">设置</div>
      </el-button>
      <el-button text type="default" size="large" @click="handleOpenPlugin">
        <template #icon>
          <component :is="renderIconFont('chajian1')" />
        </template>
        <div v-if="!collapsed">扩展管理</div>
      </el-button>
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
  flex-direction: column;
  align-items: flex-start;
}

.menu_footer :deep(.el-button) {
  justify-content: flex-start;
  width: 100%;
  margin-left: 0;
}

.menu_footer :deep(.setting-icon) {
  width: 48px;
  height: 48px;
  font-size: 48px;
}
</style>
