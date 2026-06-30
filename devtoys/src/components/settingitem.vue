<!--
 * @Author: huzhiqiang
 * @Date: 2026-06-18 14:49:55
 * @LastEditors: huzhiqiang
 * @LastEditTime: 2026-06-18 14:50:31
 * @FilePath: \devtoys\src\components\settingitem.vue
 * @Description: A component for setting, including the system settings of this software and the parameters settings of tools in this software.
-->

<script lang="ts" setup>
import type { SelectOption } from "naive-ui";
import type { Component } from "vue";
import { ref } from "vue";

type SettingSelectValue = string | number | null;

const props = defineProps<{
  icon: Component;
  title: string;
  description: string;
  type?: string;
  options?: SelectOption[];
  selectValue?: SettingSelectValue;
  switchCheckedContent?: string;
  switchUncheckedContent?: string;
  switchValue?: boolean;
  onSelectChange?: (value: SettingSelectValue) => void;
  onSwitchChange?: (value: boolean) => void;
  onNumberChange?: (value: number | null) => void;
}>();

const uuidCount = ref(1);
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
          :value="props.selectValue"
          placeholder="请选择"
          @update:value="props.onSelectChange" />
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
        <n-input-number
          v-model:value="uuidCount"
          class="win-spin"
          v-if="props.type === 'number'"
          clearable
          :min="1"
          :max="100"
          placeholder="请输入生成数量"
          @update:value="props.onNumberChange" />
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
