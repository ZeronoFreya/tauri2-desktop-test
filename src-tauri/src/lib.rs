// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}
// use std::sync::Arc;

struct WindowTracker {
    child: tauri::WebviewWindow,
    offset: (f64, f64),
}
use tauri::{Manager, PhysicalPosition, PhysicalSize, WindowEvent, WebviewWindowBuilder};
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

            let inner_size: PhysicalSize<u32> = parent.inner_size()?;

            let parent_inner_pos = parent.inner_position()?;
            let parent_outer_pos = parent.outer_position()?;

            
            let offset = (
                (parent_inner_pos.x - parent_outer_pos.x) as f64 + inner_size.width as f64,
                (parent_inner_pos.y - parent_outer_pos.y) as f64,
            );

            let child = WebviewWindowBuilder::new(
                app,
                "child-window",
                tauri::WebviewUrl::App("multiwindow.html".into()),
            )
            .inner_size(400.0, 300.0)
            .position(
                parent_inner_pos.x as f64 + inner_size.width as f64,
                parent_inner_pos.y as f64,
            )
            .parent(&parent)?
            .decorations(false)
            .shadow(false)
            .build()?;

            app.manage(WindowTracker { child, offset });

            parent.on_window_event(move |event| {
                if let WindowEvent::Moved(pos) = event {
                    if let Some(tracker) = app_handle.try_state::<WindowTracker>() {
                        let new_pos = PhysicalPosition::new(
                            pos.x as f64 + tracker.offset.0,
                            pos.y as f64 + tracker.offset.1,
                        );
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
