use tauri::{AppHandle, Emitter};
// use tauri::{Manager, PhysicalPosition, PhysicalSize, WindowEvent, WebviewWindowBuilder};
use tauri::{Manager, PhysicalPosition, WebviewWindowBuilder};
mod drop;
// mod tab;

// struct WindowTracker {
//     child: tauri::WebviewWindow,
//     offset: (f64, f64),
// }

#[tauri::command]
fn start_drag(window: tauri::Window) {
    let _ = window.start_dragging();
}

#[tauri::command]
fn close_window(window: tauri::Window) {
    let _ = window.close();
}

#[tauri::command]
async fn force_close(app_handle: AppHandle) -> Result<(), String> {
    // 执行清理工作
    // 比如保存配置、清理临时文件等

    // 退出整个应用程序，这样不会触发 CloseRequested 事件
    app_handle.exit(0);

    Ok(())
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}


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

            let inner_size = parent.inner_size().expect("Failed to get size");
            let parent_inner_pos = parent.inner_position().expect("Failed to get inner pos");
            let parent_outer_pos = parent.outer_position().expect("Failed to get outer pos");
 
            // 计算初始偏移量
            // 注意：这里的 inner_size 通常指内容区域
            let offset_x = (parent_inner_pos.x - parent_outer_pos.x) + inner_size.width as i32;
            let offset_y = parent_inner_pos.y - parent_outer_pos.y;

            let init_x = parent_outer_pos.x + offset_x;
            let init_y = parent_outer_pos.y + offset_y;

            let child = WebviewWindowBuilder::new(
                app,
                "child-window",
                tauri::WebviewUrl::App("multiwindow.html".into()),
            )
            .inner_size(400.0, 300.0)
            .position( init_x as f64, init_y as f64 )
            .parent(&parent)?
            .decorations(false)
            .shadow(false)
            // 2. 关键优化：如果不需要透明，设为 false 可大幅提升 Windows 拖拽时的重绘性能
            .transparent(false) 
            // 3. 避免子窗口出现在任务栏，保持“伪多Webview”的错觉
            .skip_taskbar(true) 
            .build()?;

            // app.manage(WindowTracker { child, offset });

            let child_clone = child.clone();
            // 使用 move 闭包，捕获 offset_x, offset_y (栈变量，极快)
            parent.on_window_event(move |event| {
                if let tauri::WindowEvent::Moved(pos) = event {
                    // pos 是 PhysicalPosition<i32>，直接整数运算
                    let new_x = pos.x + offset_x;
                    let new_y = pos.y + offset_y;
                    
                    let _ = child_clone.set_position(PhysicalPosition::new(new_x, new_y));
                }
            });

            Ok(())
        })
        .on_window_event(|window, event| match event {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                // println!(">>> 窗口标签: {}", window.label());
                api.prevent_close();
                // 发送事件到前端，让前端显示确认对话框
                if let Err(e) = window.emit("before-close", ()) {
                    eprintln!("发送 before-close 事件失败: {}", e);
                }
            }
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            start_drag,
            close_window,
            force_close,
            drop::create_drop_window,
            drop::hide_drop_window,
            drop::show_drop_window,
            drop::take_drop_files
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
