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
#[allow(dead_code)]
pub struct TargetSize {
    pub length: f64,
    pub width: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
pub struct FovParams {
    pub distance: f64,
    pub sensor_size: SensorSize,
    pub focal_length: f64,
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

struct FormatDecimal;
impl FormatDecimal {
    pub fn format_decimal(value: f64, bits: u32) -> f64 {
        let factor = 10_i32.pow(bits) as f64;
        (value * factor).round() / factor
    }
}

pub struct Sensor {
    pub params: SensorParams,
}

impl Sensor {
    pub fn new(params: SensorParams) -> Self {
        Self { params }
    }

    /**
     * 计算靶面尺寸(mm)
     */
    pub fn get_sensor_size(&self) -> TargetSize {
        let resolution = self.params.resolution;
        let sensor_size = self.params.sensor_size;

        // 保留 4 位小数
        let length =
            FormatDecimal::format_decimal(resolution.length * sensor_size.length / 1000.0, 4);
        let width = FormatDecimal::format_decimal(resolution.width * sensor_size.width / 1000.0, 4);

        TargetSize { length, width }
    }
}

pub struct Fov {
    pub params: FovParams,
}

impl Fov {
    pub fn new(params: FovParams) -> Self {
        Self { params }
    }

    /**
     * 计算视野(mm)
     */
    pub fn get_fov(&self) -> FieldOfView {
        let distance = self.params.distance;
        let sensor_size = self.params.sensor_size;
        let focal_length = self.params.focal_length;

        let len = (distance * sensor_size.length) / focal_length;
        let w = (distance * sensor_size.width) / focal_length;

        FieldOfView {
            length: FormatDecimal::format_decimal(len, 4),
            width: FormatDecimal::format_decimal(w, 4),
        }
    }
}
