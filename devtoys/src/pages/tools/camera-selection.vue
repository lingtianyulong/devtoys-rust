<script setup lang="ts"></script>

<template>
  <n-card class="main_card">
    <div class="title">相机选型</div>
    <n-space vertical :size="10">
      <n-card title="计算靶面尺寸">
        <n-form :style="{ width: '100%' }">
          <n-form-item label="分辨率(长边) x (短边) (pixel)">
            <n-input placeholder="请输入分辨率(长边)" style="width: 180px" />
            <span style="margin: 0 10px">x</span>
            <n-input placeholder="请输入分辨率(短边)" style="width: 180px" />
          </n-form-item>
          <n-form-item label="靶面尺寸(长) x (宽) (μm)">
            <n-input placeholder="请输入靶面尺寸(长)" style="width: 180px" />
            <span style="margin: 0 10px">x</span>
            <n-input placeholder="请输入靶面尺寸(宽)" style="width: 180px" />
            <n-button type="primary" style="margin-left: 10px">计算</n-button>
          </n-form-item>
          <n-form-item label="靶面尺寸(长) x (宽) (㎜)">
            <n-input readonly style="width: 180px" placeholder="" />
            <span style="margin: 0 10px">x</span>
            <n-input readonly style="width: 180px" placeholder="" />
          </n-form-item>
        </n-form>
      </n-card>

      <n-card title="计算 FOV">
        <n-form inline :style="{ width: '100%', marginTop: '20px' }">
          <n-form-item label="工作距离(mm)">
            <n-input style="width: 180px" placeholder="输入相机的工作距离" />
          </n-form-item>
          <n-form-item label="靶面尺寸(mm)">
            <n-input style="width: 180px" placeholder="输入长边靶面尺寸" />
            <span style="margin: 0 10px">x</span>
            <n-input style="width: 180px" placeholder="输入短边靶面尺寸" />
          </n-form-item>
          <n-form-item label="焦距(mm)">
            <n-input style="width: 180px" placeholder="输入焦距" />
          </n-form-item>
        </n-form>
        <n-button type="primary" style="margin-left: 10px">计算</n-button>
        <n-form inline :style="{ width: '100%', marginTop: '20px' }">
          <n-form-item label="长边视野(mm)">
            <n-input readonly style="width: 180px" placeholder="" />
          </n-form-item>
          <n-form-item label="短边视野(mm)">
            <n-input readonly style="width: 180px" placeholder="" />
          </n-form-item>
        </n-form>
      </n-card>
      <n-card title="计算景深">
        <n-form inline :style="{ width: '100%', marginTop: '20px' }">
          <n-form-item label="光圈值(f)">
            <n-input style="width: 180px" placeholder="输入光圈值" />
          </n-form-item>
          <n-form-item label="弥散圆(mm)">
            <n-input style="width: 180px" placeholder="输入弥散圆" />
          </n-form-item>
        </n-form>
        <n-button type="primary" style="margin-left: 10px">计算</n-button>
        <n-form inline :style="{ width: '100%', marginTop: '20px' }">
          <n-form-item label="长边视野(mm)">
            <n-input readonly style="width: 180px" placeholder="" />
          </n-form-item>
          <n-form-item label="短边视野(mm)">
            <n-input readonly style="width: 180px" placeholder="" />
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
