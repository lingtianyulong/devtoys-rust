use crate::uuid::uuid::UUIDGenerator;
use tauri_plugin_log::log;

#[tauri::command]
pub fn generate_uuid(params: String) -> String {
    let generator = UUIDGenerator::new(&params);
    let uuids = generator.generate();
    let json = match serde_json::to_string(&uuids) {
        Ok(json) => json,
        Err(e) => {
            log::error!("Failed to convert UUIDs to JSON: {}", e);
            return "".to_string();
        }
    };
    json
}
