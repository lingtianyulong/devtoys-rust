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
  numberValue?: number | null;
  onSelectChange?: (value: SettingSelectValue) => void;
  onSwitchChange?: (value: boolean) => void;
  onNumberChange?: (value: number | null) => void;
}>();
</script>

<template>
  <el-card class="settingitem_card">
    <div class="setting-row">
      <div class="left">
        <component :is="props.icon" />
      </div>
      <div class="center">
        <div class="title">{{ props.title }}</div>
        <div class="description">{{ props.description }}</div>
      </div>
      <div class="right">
        <el-select
          v-if="props.type === 'select'"
          :model-value="props.selectValue"
          @change="props.onSelectChange">
          <el-option
            v-for="option in props.options"
            :key="option.value"
            :label="option.label"
            :value="option.value" />
        </el-select>
        <el-switch
          v-if="props.type === 'switch'"
          v-model="props.switchValue"
          @change="props.onSwitchChange"
          :active-text="props.switchCheckedContent"
          :inactive-text="props.switchUncheckedContent"
          :active-value="true"
          :inactive-value="false">
        </el-switch>
        <el-input-number
          v-if="props.type === 'number'"
          v-model="props.numberValue"
          controls-position="right"
          clearable
          :min="1"
          :max="100"
          :step="1"
          @change="props.onNumberChange" />
      </div>
    </div>
  </el-card>
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
