use tauri::{Manager, PhysicalPosition};
use winapi::um::windef::HWND;
use winapi::um::winuser::{SetWindowLongPtrW, GWLP_HWNDPARENT};

#[tauri::command]
pub fn create_child_window(app: tauri::AppHandle, parent: tauri::Window) {
    let child = tauri::WebviewWindowBuilder::new(
        &app,
        "child-window",
        tauri::WebviewUrl::App("about:blank".into()),
    ).inner_size(400.0, 300.0)
    .build()
    .unwrap();

    // 获取原生窗口句柄
    let parent_hwnd = parent.hwnd().unwrap() as HWND;
    let child_hwnd = child.hwnd().unwrap() as HWND;

    // 设置父子关系
    unsafe {
        SetWindowLongPtrW(child_hwnd, GWLP_HWNDPARENT, parent_hwnd as _);
    }

    // 初始位置偏移
    let parent_pos = parent.outer_position().unwrap();
    child
        .set_position(PhysicalPosition {
            x: parent_pos.x + 50,
            y: parent_pos.y + 50,
        })
        .unwrap();
}
