<script setup lang="ts">
import {
  ElButton,
  ElCard,
  ElForm,
  ElFormItem,
  ElInput,
  ElSpace,
} from "element-plus";
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

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

const handleNumberKeydown = (event: Event) => {
  if (!(event instanceof KeyboardEvent)) {
    return;
  }

  if (
    event.ctrlKey ||
    event.metaKey ||
    event.altKey ||
    event.key.length !== 1
  ) {
    return;
  }

  const input = event.target as HTMLInputElement;
  const start = input.selectionStart ?? input.value.length;
  const end = input.selectionEnd ?? input.value.length;
  const nextValue =
    input.value.slice(0, start) + event.key + input.value.slice(end);

  if (!onlyNumber(nextValue)) {
    event.preventDefault();
  }
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

const calculateSensorSize = async () => {
  let params = {
    resolution: {
      length: Number(sensorForm.value.resolution.length),
      width: Number(sensorForm.value.resolution.width),
    },
    sensor_size: {
      length: Number(sensorForm.value.sensorSize.length),
      width: Number(sensorForm.value.sensorSize.width),
    },
  };

  const json = JSON.stringify(params);
  console.log("json", json);

  await invoke<string>("get_sensor_size", { sensorParams: json }).then(
    (result) => {
      console.log("result", result);
      sensorForm.value.targetSize = JSON.parse(result);
    },
  );
};

const calculateFov = async () => {
  const distance = FovForm.value.distance;
  const sensorSize = FovForm.value.sensorSize;
  const focalLength = FovForm.value.focalLength;

  const params = {
    distance: Number(distance),
    sensor_size: {
      length: Number(sensorSize.length),
      width: Number(sensorSize.width),
    },
    focal_length: Number(focalLength),
  };
  const json = JSON.stringify(params);
  console.log("json", json);

  await invoke<string>("get_fov", { fovParams: json }).then((result) => {
    console.log("result", result);
    FovForm.value.fov = JSON.parse(result);
  });
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
  <el-card class="main_card">
    <div class="title">相机选型</div>
    <el-space direction="vertical" :size="10" fill style="width: 100%">
      <el-card header="计算靶面尺寸">
        <el-form
          label-position="right"
          label-width="200px"
          :model="sensorForm"
          :style="{ width: '100%' }">
          <el-form-item label="分辨率(长边) x (短边) (pixel)">
            <el-input
              v-model="sensorForm.resolution.length"
              placeholder="请输入分辨率(长边)"
              style="width: 180px"
              @keydown="handleNumberKeydown" />
            <span style="margin: 0 10px">x</span>
            <el-input
              v-model="sensorForm.resolution.width"
              placeholder="请输入分辨率(短边)"
              style="width: 180px"
              @keydown="handleNumberKeydown" />
          </el-form-item>
          <el-form-item label="靶面尺寸(长) x (宽) (μm)">
            <el-input
              v-model="sensorForm.sensorSize.length"
              placeholder="请输入靶面尺寸(长)"
              style="width: 180px"
              @keydown="handleNumberKeydown" />
            <span style="margin: 0 10px">x</span>
            <el-input
              v-model="sensorForm.sensorSize.width"
              placeholder="请输入靶面尺寸(宽)"
              style="width: 180px"
              @keydown="handleNumberKeydown" />
            <el-button
              type="primary"
              style="margin-left: 10px"
              @click="calculateSensorSize">
              计算
            </el-button>
          </el-form-item>
          <el-form-item label="靶面尺寸(长) x (宽) (㎜)">
            <el-input
              readonly
              style="width: 180px"
              placeholder=""
              v-model="sensorForm.targetSize.length" />
            <span style="margin: 0 10px">x</span>
            <el-input
              readonly
              style="width: 180px"
              placeholder=""
              v-model="sensorForm.targetSize.width" />
          </el-form-item>
        </el-form>
      </el-card>

      <el-card class="content_card" header="计算 FOV">
        <el-form :model="FovForm" inline :style="{ width: '100%' }">
          <el-form-item label="工作距离(mm)">
            <el-input
              v-model="FovForm.distance"
              style="width: 180px"
              placeholder="输入相机的工作距离"
              @keydown="handleNumberKeydown" />
          </el-form-item>
          <el-form-item label="靶面尺寸(mm)">
            <el-input
              v-model="FovForm.sensorSize.length"
              style="width: 180px"
              placeholder="输入长边靶面尺寸"
              @keydown="handleNumberKeydown" />
            <span style="margin: 0 10px">x</span>
            <el-input
              v-model="FovForm.sensorSize.width"
              style="width: 180px"
              placeholder="输入短边靶面尺寸"
              @keydown="handleNumberKeydown" />
          </el-form-item>
          <el-form-item label="焦距(mm)">
            <el-input
              v-model="FovForm.focalLength"
              style="width: 180px"
              placeholder="输入焦距"
              @keydown="handleNumberKeydown" />
          </el-form-item>
        </el-form>
        <el-button
          type="primary"
          style="margin-top: 10px"
          @click="calculateFov">
          计算
        </el-button>
        <el-form inline :style="{ width: '100%', marginTop: '20px' }">
          <el-form-item label="长边视野(mm)">
            <el-input
              readonly
              style="width: 180px"
              placeholder=""
              v-model="FovForm.fov.length" />
          </el-form-item>
          <el-form-item label="短边视野(mm)">
            <el-input
              readonly
              style="width: 180px"
              placeholder=""
              v-model="FovForm.fov.width" />
          </el-form-item>
        </el-form>
      </el-card>
      <el-card class="content_card" header="计算景深">
        <el-form :model="DepthOfFieldForm" inline :style="{ width: '100%' }">
          <el-form-item label="光圈值(f)">
            <el-input
              v-model="DepthOfFieldForm.fNumber"
              style="width: 180px"
              placeholder="输入光圈值"
              @keydown="handleNumberKeydown" />
          </el-form-item>
          <el-form-item label="弥散圆(μm)">
            <el-input
              v-model="DepthOfFieldForm.circleOfConfusion"
              style="width: 180px"
              placeholder="输入弥散圆"
              @keydown="handleNumberKeydown" />
          </el-form-item>
          <el-form-item label="工作距离(mm)">
            <el-input
              v-model="DepthOfFieldForm.distance"
              style="width: 180px"
              placeholder="输入工作距离"
              @keydown="handleNumberKeydown" />
          </el-form-item>
          <el-form-item label="焦距(mm)">
            <el-input
              v-model="DepthOfFieldForm.focalLength"
              style="width: 180px"
              placeholder="输入焦距"
              @keydown="handleNumberKeydown" />
          </el-form-item>
        </el-form>
        <el-button
          type="primary"
          style="margin-top: 10px"
          @click="calculateDepthOfField">
          计算
        </el-button>
        <el-form inline :style="{ width: '100%', marginTop: '20px' }">
          <el-form-item label="景深(近/远)(mm)">
            <el-input
              readonly
              style="width: 180px"
              placeholder=""
              v-model="DepthOfFieldForm.depthOfField" />
          </el-form-item>
        </el-form>
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
  overflow-y: auto;
}

.main_card :deep(.el-input__inner) {
  border-radius: 0;
  box-shadow: none;
}

.main_card :deep(.el-input__inner::selection) {
  border-radius: 0;
}

.main_card :deep(.el-card__body) {
  padding-bottom: 20px;
}

.content_card :deep(.el-card__body) {
  padding-bottom: 20px;
}

.content_card :deep(.el-form-item) {
  margin-bottom: 0;
}

.content_card :deep(.el-form:last-of-type) {
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
