// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

mod drop;
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_drag::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            drop::create_drop_window,
            drop::hide_drop_window,
            drop::show_drop_window,
            drop::update_drop_files,
            drop::take_drop_files
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
