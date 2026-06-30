// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

use chrono::Local;

mod uuid;
use uuid::commands::generate_uuid;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let log_plugin = tauri_plugin_log::Builder::new()
        .max_file_size(1024 * 1024 * 10)
        .level(tauri_plugin_log::log::LevelFilter::Info)
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
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(
            tauri_plugin_log::Builder::new()
                .level(tauri_plugin_log::log::LevelFilter::Info)
                .build(),
        )
        .plugin(log_plugin)
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![generate_uuid])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
