<script setup lang="ts">
import { ref } from "vue";

// 分辨率
type Resolution = {
  length: string; // 长边分辨率
  width: string; // 短边分辨率
};

// 靶面尺寸
type SensorSize = {
  length: string; // 长边靶面尺寸
  width: string; // 短边靶面尺寸
};

// 目标尺寸
type TargetSize = {
  length: string; // 长边目标尺寸
  width: string; // 短边目标尺寸
};

// 视野
type Fov = {
  length: string; // 长边视野
  width: string; // 短边视野
};

const sensorForm = ref({
  resolution: {
    length: "",
    width: "",
  } as Resolution,
  sensorSize: {
    length: "",
    width: "",
  } as SensorSize,
  targetSize: {
    length: "",
    width: "",
  } as TargetSize,
});

const FovForm = ref({
  distance: "", // 工作距离
  sensorSize: {
    length: "",
    width: "",
  } as SensorSize,
  focalLength: "", // 焦距
  fov: {
    length: "",
    width: "",
  } as Fov,
});

const DepthOfFieldForm = ref({
  fNumber: "", // 光圈值
  circleOfConfusion: "", // 弥散圆
  distance: "", // 工作距离
  focalLength: "", // 焦距
  depthOfField: "", // 景深
});

const onlyNumber = (value: string) => {
  return /^-?\d*\.?\d*$/.test(value);
};

const formatTo4Decimals = (num: number) => {
  const str = num.toString();
  if (str.includes(".")) {
    const decimalPart = str.split(".")[1];
    if (decimalPart.length > 4) {
      return num.toFixed(4);
    }
  }
  return str;
};

const calculateSensorSize = () => {
  const resolution = sensorForm.value.resolution;
  const sensorSize = sensorForm.value.sensorSize;

  const lengthValue =
    (Number(resolution.length) * Number(sensorSize.length)) / 1000.0;
  const widthValue =
    (Number(resolution.width) * Number(sensorSize.width)) / 1000.0;
  console.log("lengthValue", lengthValue);
  console.log("widthValue", widthValue);
  sensorForm.value.targetSize = {
    length: formatTo4Decimals(lengthValue),
    width: formatTo4Decimals(widthValue),
  };
};

const calculateFov = () => {
  const distance = FovForm.value.distance;
  const sensorSize = FovForm.value.sensorSize;
  const focalLength = FovForm.value.focalLength;

  const lenFov =
    (Number(distance) * Number(sensorSize.length)) / Number(focalLength);
  const widthFov =
    (Number(distance) * Number(sensorSize.width)) / Number(focalLength);
  FovForm.value.fov = {
    length: formatTo4Decimals(lenFov),
    width: formatTo4Decimals(widthFov),
  };
};

const calculateDepthOfField = () => {
  // 景深 = 2 * (光圈值 * 弥散圆 * 工作距离^2) / (焦距^2)
  // 此处景深使用简化公式进行近似求值
  const fNumber = DepthOfFieldForm.value.fNumber;
  const circleOfConfusion = DepthOfFieldForm.value.circleOfConfusion;
  const distance = DepthOfFieldForm.value.distance;
  const focalLength = DepthOfFieldForm.value.focalLength;
  const depthOfField =
    (2 *
      Number(fNumber) *
      (Number(circleOfConfusion) / 1000) *
      Math.pow(Number(distance), 2)) /
    Math.pow(Number(focalLength), 2);

  DepthOfFieldForm.value.depthOfField = formatTo4Decimals(depthOfField);
};
</script>

<template>
  <n-card class="main_card">
    <div class="title">相机选型</div>
    <n-space vertical :size="10">
      <n-card title="计算靶面尺寸">
        <n-form :model="sensorForm" :style="{ width: '100%' }">
          <n-form-item label="分辨率(长边) x (短边) (pixel)">
            <n-input
              v-model:value="sensorForm.resolution.length"
              placeholder="请输入分辨率(长边)"
              style="width: 180px"
              :allow-input="onlyNumber" />
            <span style="margin: 0 10px">x</span>
            <n-input
              v-model:value="sensorForm.resolution.width"
              placeholder="请输入分辨率(短边)"
              style="width: 180px"
              :allow-input="onlyNumber" />
          </n-form-item>
          <n-form-item label="靶面尺寸(长) x (宽) (μm)">
            <n-input
              v-model:value="sensorForm.sensorSize.length"
              placeholder="请输入靶面尺寸(长)"
              style="width: 180px"
              :allow-input="onlyNumber" />
            <span style="margin: 0 10px">x</span>
            <n-input
              v-model:value="sensorForm.sensorSize.width"
              placeholder="请输入靶面尺寸(宽)"
              style="width: 180px"
              :allow-input="onlyNumber" />
            <n-button
              type="primary"
              style="margin-left: 10px"
              @click="calculateSensorSize">
              计算
            </n-button>
          </n-form-item>
          <n-form-item label="靶面尺寸(长) x (宽) (㎜)">
            <n-input
              readonly
              style="width: 180px"
              placeholder=""
              v-model:value="sensorForm.targetSize.length" />
            <span style="margin: 0 10px">x</span>
            <n-input
              readonly
              style="width: 180px"
              placeholder=""
              v-model:value="sensorForm.targetSize.width" />
          </n-form-item>
        </n-form>
      </n-card>

      <n-card class="content_card" title="计算 FOV">
        <n-form :model="FovForm" inline :style="{ width: '100%' }">
          <n-form-item label="工作距离(mm)">
            <n-input
              v-model:value="FovForm.distance"
              style="width: 180px"
              placeholder="输入相机的工作距离"
              :allow-input="onlyNumber" />
          </n-form-item>
          <n-form-item label="靶面尺寸(mm)">
            <n-input
              v-model:value="FovForm.sensorSize.length"
              style="width: 180px"
              placeholder="输入长边靶面尺寸"
              :allow-input="onlyNumber" />
            <span style="margin: 0 10px">x</span>
            <n-input
              v-model:value="FovForm.sensorSize.width"
              style="width: 180px"
              placeholder="输入短边靶面尺寸"
              :allow-input="onlyNumber" />
          </n-form-item>
          <n-form-item label="焦距(mm)">
            <n-input
              v-model:value="FovForm.focalLength"
              style="width: 180px"
              placeholder="输入焦距"
              :allow-input="onlyNumber" />
          </n-form-item>
        </n-form>
        <n-button type="primary" style="margin-top: 10px" @click="calculateFov">
          计算
        </n-button>
        <n-form inline :style="{ width: '100%', marginTop: '20px' }">
          <n-form-item label="长边视野(mm)">
            <n-input
              readonly
              style="width: 180px"
              placeholder=""
              v-model:value="FovForm.fov.length" />
          </n-form-item>
          <n-form-item label="短边视野(mm)">
            <n-input
              readonly
              style="width: 180px"
              placeholder=""
              v-model:value="FovForm.fov.width" />
          </n-form-item>
        </n-form>
      </n-card>
      <n-card class="content_card" title="计算景深">
        <n-form :model="DepthOfFieldForm" inline :style="{ width: '100%' }">
          <n-form-item label="光圈值(f)">
            <n-input
              v-model:value="DepthOfFieldForm.fNumber"
              style="width: 180px"
              placeholder="输入光圈值"
              :allow-input="onlyNumber" />
          </n-form-item>
          <n-form-item label="弥散圆(μm)">
            <n-input
              v-model:value="DepthOfFieldForm.circleOfConfusion"
              style="width: 180px"
              placeholder="输入弥散圆"
              :allow-input="onlyNumber" />
          </n-form-item>
          <n-form-item label="工作距离(mm)">
            <n-input
              v-model:value="DepthOfFieldForm.distance"
              style="width: 180px"
              placeholder="输入工作距离"
              :allow-input="onlyNumber" />
          </n-form-item>
          <n-form-item label="焦距(mm)">
            <n-input
              v-model:value="DepthOfFieldForm.focalLength"
              style="width: 180px"
              placeholder="输入焦距"
              :allow-input="onlyNumber" />
          </n-form-item>
        </n-form>
        <n-button
          type="primary"
          style="margin-top: 10px"
          @click="calculateDepthOfField">
          计算
        </n-button>
        <n-form inline :style="{ width: '100%', marginTop: '20px' }">
          <n-form-item label="景深(近/远)(mm)">
            <n-input
              readonly
              style="width: 180px"
              placeholder=""
              v-model:value="DepthOfFieldForm.depthOfField" />
          </n-form-item>
        </n-form>
      </n-card>
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
  overflow-y: auto;
}

.main_card :deep(.n-input__input-el) {
  border-radius: 0;
  box-shadow: none;
}

.main_card :deep(.n-input__input-el::selection) {
  border-radius: 0;
}

.main_card :deep(.n-card > .n-card__content) {
  padding-bottom: 20px;
}

.main_card :deep(.n-form-item:last-child .n-form-item-feedback-wrapper) {
  min-height: 0;
}

.content_card :deep(.n-card > .n-card__content) {
  padding-bottom: 20px;
}

.content_card :deep(.n-form-item .n-form-item-feedback-wrapper) {
  min-height: 0;
}

.content_card :deep(.n-form:last-of-type) {
  margin-bottom: 0;
}

.title {
  font-size: 24px;
  font-weight: bold;
  text-align: center;
  margin-bottom: 20px;
  color: #333;
  display: flex;
  justify-content: start;
  align-items: center;
}
</style>
