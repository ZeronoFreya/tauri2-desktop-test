// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
// use std::sync::Arc;

struct WindowTracker {
    child: tauri::WebviewWindow,
    offset: (i32, i32),
}
use tauri::{Manager, PhysicalPosition, WindowEvent};
mod drop;
// mod tab;
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_drag::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_positioner::init())
        .setup(|app| {
            let parent = app
                .get_webview_window("main")
                .ok_or("Main window not found")?;

            let app_handle = app.handle().clone();

            // 计算初始偏移
            let parent_inner_pos = parent.inner_position()?;
            let parent_outer_pos = parent.outer_position()?;


            let offset = (
                parent_inner_pos.x - parent_outer_pos.x,
                parent_inner_pos.y - parent_outer_pos.y,
            );

            // 创建子窗口
            let child = tauri::WebviewWindowBuilder::new(
                app,
                "child-window",
                tauri::WebviewUrl::App("about:blank".into()),
            )
            .inner_size(400.0, 300.0)
            .position(parent_inner_pos.x as f64, parent_inner_pos.y as f64)
            .parent(&parent)?
            .decorations(false)
            .shadow(false)
            .build()?;

            // 存储跟踪器
            app.manage(WindowTracker { child, offset });

            // 监听父窗口移动事件
            parent.on_window_event(move |event| {
                if let WindowEvent::Moved(pos) = event {
                    if let Some(tracker) = app_handle.try_state::<WindowTracker>() {
                        let new_pos = PhysicalPosition {
                            x: pos.x + tracker.offset.0,
                            y: pos.y + tracker.offset.1,
                        };
                        let _ = tracker.child.set_position(new_pos);
                    }
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            drop::create_drop_window,
            drop::hide_drop_window,
            drop::show_drop_window,
            drop::take_drop_files
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
