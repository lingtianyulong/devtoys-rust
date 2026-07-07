/**
 * 相机选型相关功能及工具
 */
use serde::{Deserialize, Serialize};

/*
 * 分辨率
 */
#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
pub struct Resolution {
    pub length: f64,
    pub width: f64,
}

/*
 * 靶面尺寸
 */
#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
pub struct SensorSize {
    pub length: f64,
    pub width: f64,
}

/*
 * 目标尺寸
 */
#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
pub struct TargetSize {
    pub length: f64,
    pub width: f64,
}

/*
 * 视野
 */
#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
pub struct FieldOfView {
    pub length: f64,
    pub width: f64,
}

/*
 * 相机传感器尺寸
 */
#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
pub struct SensorParams {
    pub resolution: Resolution,  // 分辨率
    pub sensor_size: SensorSize, // 传感器尺寸 （um)
}

pub struct Sensor {
    pub params: SensorParams,
}

impl Sensor {
    pub fn new(params: SensorParams) -> Self {
        Self { params }
    }

    // 保留 4 位小数
    fn factor_conversion(value: f64) -> f64 {
        (value * 10000.0).round() / 10000.0
    }

    pub fn get_sensor_size(&self) -> TargetSize {
        let resolution = self.params.resolution;
        let sensor_size = self.params.sensor_size;

        // 保留 4 位小数
        let length = Self::factor_conversion(resolution.length * sensor_size.length / 1000.0);
        let width = Self::factor_conversion(resolution.width * sensor_size.width / 1000.0);

        TargetSize { length, width }
    }
}

// pub struct CameraSelection {
//     pub sensorParams: SensorParams,
//     pub targetSize: TargetSize,   // 目标尺寸
//     pub fieldOfView: FieldOfView, // 视野
//     pub depthOfField: f64,        // 相机深度(mm)
// }

// impl CameraSelection {
//     pub fn new(sensorParams: SensorParams) -> Self {
//         Self { sensorParams }
//     }
// }
