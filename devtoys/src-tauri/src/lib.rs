// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

use chrono::Local;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let log_plugin = tauri_plugin_log::Builder::new()
        .format(|out, message, record| {
            let now = Local::now();
            let formatted = now.format("%Y-%m-%d %H:%M:%S%.3f").to_string();
            out.finish(format_args!(
                "{} [{}] - {}",
                formatted,
                record.level(),
                message
            ));
        })
        .clear_targets()
        .target(tauri_plugin_log::Target::new(
            tauri_plugin_log::TargetKind::Folder {
                path: std::path::PathBuf::from("logs"),
                file_name: Some("app.log".to_string()),
            },
        ))
        .build();

    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::new()
                .level(tauri_plugin_log::log::LevelFilter::Info)
                .build(),
        )
        .plugin(log_plugin)
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
