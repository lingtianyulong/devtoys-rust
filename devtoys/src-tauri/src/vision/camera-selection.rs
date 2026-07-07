/**
 * 相机选型相关功能及工具
 */
use serde::{Deserialize, Serialize};

/*
 * 分辨率
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub struct Resolution {
    pub length: f64,
    pub width: f64,
}

/*
 * 靶面尺寸
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub struct SensorSize {
    pub length: f64,
    pub width: f64,
}

/*
 * 目标尺寸
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub struct TargetSize {
    pub length: f64,
    pub width: f64,
}

/*
 * 视野
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub struct FieldOfView {
    pub length: f64,
    pub width: f64,
}

/*
 * 相机传感器尺寸
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub struct SensorParams {
    pub resolution: Resolution, // 分辨率
    pub sensorSize: SensorSize, // 传感器尺寸 （um)
}

pub struct CameraSelection {
    pub sensorParams: SensorParams,
    pub targetSize: TargetSize,   // 目标尺寸
    pub fieldOfView: FieldOfView, // 视野
    pub depthOfField: f64,        // 相机深度(mm)
}

impl CameraSelection {
    pub fn new(sensorParams: SensorParams) -> Self {
        Self { sensorParams }
    }
}
