<!--
 * @Author: lizhijun lizhijun@tsign.cn
 * @Date: 2026-06-18 14:49:55
 * @LastEditors: lizhijun lizhijun@tsign.cn
 * @LastEditTime: 2026-06-18 14:50:31
 * @FilePath: \devtoys\src\components\settingitem.vue
 * @Description: 用于封装设置项的组件, 主要在设置页面中使用
-->

<script lang="ts" setup>
import type { SelectOption } from "naive-ui";

const props = defineProps<{
  icon: Object;
  title: string;
  description: string;
  type?: string;
  options?: SelectOption[];
  switchCheckedContent?: string;
  switchUncheckedContent?: string;
  switchValue?: boolean;
  onSwitchChange?: (value: boolean) => void;
}>();
</script>

<template>
  <n-card class="settingitem_card">
    <div class="setting-row">
      <div class="left">
        <component :is="props.icon" />
      </div>
      <div class="center">
        <div class="title">{{ props.title }}</div>
        <div class="description">{{ props.description }}</div>
      </div>
      <div class="right">
        <n-select
          v-if="props.type === 'select'"
          :options="props.options"
          placeholder="请选择" />
        <n-switch
          v-if="props.type === 'switch'"
          :value="props.switchValue"
          @update:value="props.onSwitchChange">
          <template #checked>
            <span>{{ props.switchCheckedContent }}</span>
          </template>
          <template #unchecked>
            <span>{{ props.switchUncheckedContent }}</span>
          </template>
        </n-switch>
      </div>
    </div>
  </n-card>
</template>

<style scoped>
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
  color: #333;
}

.description {
  font-size: 14px;
  color: #666;
}

.right {
  width: 180px;
  display: flex;
  justify-content: flex-end;
  align-items: center;
}
</style>
