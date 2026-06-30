use serde::{Deserialize, Serialize};
use tauri_plugin_log::log;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UUIDParams {
    hyphen: bool,    // 是否包含连字符
    uppercase: bool, // 是否使用大写字母
    version: String, // UUID 版本
    count: u32,      // 生成数量
}

impl Default for UUIDParams {
    fn default() -> Self {
        Self {
            hyphen: true,
            uppercase: false,
            version: "v4".to_string(),
            count: 1,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UUIDGenerator {
    params: UUIDParams,
}

impl UUIDGenerator {
    pub fn new(params: &str) -> Self {
        let params = match serde_json::from_str(params) {
            Ok(params) => params,
            Err(e) => {
                log::error!("Failed to parse UUID params: {}", e);
                return Self {
                    params: UUIDParams::default(),
                };
            }
        };

        Self { params }
    }
}

impl UUIDGenerator {
    pub fn generate(&self) -> Vec<String> {
        let mut uuids = Vec::new();
        for _ in 0..self.params.count {
            let uuid = match self.params.version.as_str() {
                "v4" => Uuid::new_v4(),
                "v7" => Uuid::now_v7(),
                _ => Uuid::new_v4(),
            };

            let mut uuid_str = uuid.to_string();
            if !self.params.hyphen {
                uuid_str = uuid_str.replace("-", "");
            }
            if self.params.uppercase {
                uuid_str = uuid_str.to_uppercase();
            }
            uuids.push(uuid_str);
        }
        uuids
    }
}
