use serde::{Deserialize, Serialize};

use std::sync::LazyLock;
use std::sync::Mutex;
use tauri::Manager; // 用于 get_window 等方法

/// 表示文件或目录的信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInfo {
    /// 文件或目录的路径
    pub path: String,
    /// 是否为目录
    pub is_dir: bool,
}

static DROP_FILES: LazyLock<Mutex<Vec<FileInfo>>> = LazyLock::new(|| Mutex::new(Vec::new()));

/// 创建一个用于文件拖放的窗口
#[tauri::command]
pub async fn create_drop_window(app: tauri::AppHandle) -> Result<(), String> {
    if app.get_webview_window("drop-window").is_some() {
        return Ok(());
    }
    // let parent_window = app
    //     .get_webview_window("main")
    //     .ok_or_else(|| "找不到主窗口".to_string())?;

    let drop_window = tauri::WebviewWindowBuilder::new(
        &app,
        "drop-window",
        tauri::WebviewUrl::App("about:blank".into()),
    )
    .title("Drop Window")
    .inner_size(400.0, 300.0)
    // .parent(&parent_window)
    // .map_err(|e| format!("设置父窗口失败: {}", e))?
    .decorations(false)
    .shadow(false)
    .always_on_top(true)
    .transparent(true)
    .skip_taskbar(true)
    .visible(false)
    //   .devtools(true)
    .build()
    .map_err(|e| format!("Failed to create window: {}", e))?;
    setup_drop_window_events(&drop_window, app.clone());
    Ok(())
}

use tauri::{DragDropEvent, WindowEvent};

fn setup_drop_window_events(window: &tauri::WebviewWindow, app: tauri::AppHandle) {
    // 文件拖放事件（Tauri 2.0新API）
    window.on_window_event(move |event| {
        if let WindowEvent::DragDrop(drag_event) = event {
            match drag_event {
                DragDropEvent::Enter { paths, position: _ } => {
                    // println!("文件进入: {:?}", paths);
                    let mut file_infos = Vec::new();
                    for path in paths {
                        let is_dir = path.is_dir();
                        file_infos.push(FileInfo {
                            path: path.to_string_lossy().into_owned(), // 转成 String
                            is_dir,
                        });
                    }
                    if let Ok(mut drop_files) = DROP_FILES.lock() {
                        *drop_files = file_infos;
                        drop(drop_files); // 立即释放锁
                        app.get_webview_window("drop-window")
                            .and_then(|w| w.hide().ok());
                    }
                }
                _ => {}
            }
        }
    });
}

/// 更新拖拽的文件列表到 DROP_FILES
///
/// # 参数
/// * `files` - 数组，每个元素为文件路径字符串
// #[tauri::command]
// pub fn update_drop_files(files: Vec<String>) -> Result<(), String> {
//     let mut file_infos = Vec::new();
//     for path_str in files {
//         let is_dir = Path::new(&path_str).is_dir();
//         file_infos.push(FileInfo {
//             path: path_str,
//             is_dir,
//         });
//     }
//     let mut drop_files = DROP_FILES.lock().map_err(|e| e.to_string())?;
//     *drop_files = file_infos;
//     Ok(())
// }

/// 获取并清空拖放文件列表
#[tauri::command]
pub fn take_drop_files() -> Result<Vec<FileInfo>, String> {
    let mut drop_files = DROP_FILES.lock().map_err(|e| e.to_string())?;
    Ok(std::mem::take(&mut *drop_files)) // 清空并返回原有数据
}

#[tauri::command]
pub async fn hide_drop_window(app: tauri::AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("drop-window") {
        window.hide().map_err(|e| e.to_string())?;
    }
    Ok(())
}

/// 显示拖拽文件窗口, 覆盖到主窗口
#[tauri::command]
pub async fn show_drop_window(app: tauri::AppHandle) -> Result<(), String> {
    // println!("show_drop_window");
    if let Some(window) = app.get_webview_window("drop-window") {
        if let Some(main_window) = app.get_webview_window("main") {
            let position = main_window.outer_position().map_err(|e| e.to_string())?;
            let size = main_window.outer_size().map_err(|e| e.to_string())?;

            window.set_position(position).map_err(|e| e.to_string())?;
            window.set_size(size).map_err(|e| e.to_string())?;
        }
        window.show().map_err(|e| e.to_string())?;
    }
    Ok(())
}
