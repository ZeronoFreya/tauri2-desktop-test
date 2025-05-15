// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// use tauri::{Manager, PhysicalPosition};
// use winapi::um::winuser::{SetWindowLongPtrW, GWLP_HWNDPARENT};
// use winapi::um::windef::HWND;

// #[tauri::command]
// fn create_child_window(parent: tauri::Window) {
//     let child = WebviewWindow::builder()
//         .label("child")
//         .url("child.html")
//         .inner_size(300, 200)
//         .build()
//         .unwrap();

//     // 获取原生窗口句柄
//     let parent_hwnd = parent.hwnd().unwrap() as HWND;
//     let child_hwnd = child.hwnd().unwrap() as HWND;

//     // 设置父子关系
//     unsafe {
//         SetWindowLongPtrW(child_hwnd, GWLP_HWNDPARENT, parent_hwnd as _);
//     }

//     // 初始位置偏移
//     let parent_pos = parent.outer_position().unwrap();
//     child.set_position(PhysicalPosition {
//         x: parent_pos.x + 50,
//         y: parent_pos.y + 50,
//     }).unwrap();
// }

mod drop;
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_drag::init())
        .plugin(tauri_plugin_dialog::init())
        // .setup(|app| {
        //     let main_window = app.get_window("main").unwrap();
        //     create_child_window(&main_window);
        //     Ok(())
        // })
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
