use crate::vision::camera_selection::*;
use serde_json;
use tauri_plugin_log::log;

#[tauri::command]
pub fn get_sensor_size(sensor_params: String) -> Result<String, String> {
    let params = match serde_json::from_str::<SensorParams>(&sensor_params) {
        Ok(params) => params,
        Err(e) => return Err(format!("Failed to parse sensor params: {}", e)),
    };

    log::info!("sensor_params: {:?}", params);

    let sensor = Sensor::new(params);
    let sensor_size = sensor.get_sensor_size();
    let sensor_size_json = match serde_json::to_string(&sensor_size) {
        Ok(json) => json,
        Err(e) => return Err(format!("Failed to serialize sensor size: {}", e)),
    };
    Ok(sensor_size_json)
}

#[tauri::command]
pub fn get_fov(fov_params: String) -> Result<String, String> {
    let params = match serde_json::from_str::<FovParams>(&fov_params) {
        Ok(params) => params,
        Err(e) => return Err(format!("Failed to parse fov params: {}", e)),
    };

    let fov = Fov::new(params);
    let fov_json = match serde_json::to_string(&fov.get_fov()) {
        Ok(json) => json,
        Err(e) => return Err(format!("Failed to serialize fov: {}", e)),
    };
    Ok(fov_json)
}
