use tauri::Manager;
use std::sync::Mutex;
use std::sync::LazyLock;
use std::path::Path;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileInfo {
    pub path: String,
    pub is_dir: bool,
}

static DROP_FILES: LazyLock<Mutex<Vec<FileInfo>>> = LazyLock::new(|| Mutex::new(Vec::new()));


#[tauri::command]
pub async fn create_drop_window(app: tauri::AppHandle) -> Result<(), String>{
  if app.get_webview_window("drop-window").is_some() {
      return Ok(());
  }
  let parent_window = app.get_webview_window("main")
        .ok_or_else(|| "找不到主窗口".to_string())?;

  tauri::WebviewWindowBuilder::new(
        &app, "drop-window", 
        tauri::WebviewUrl::App("drop-window.html".into()))
  .title("Drop Window")
  .inner_size(400.0, 300.0)
  .parent(&parent_window)
  .map_err(|e| format!("设置父窗口失败: {}", e))?
  .decorations(false)
  .shadow(false)
  .always_on_top(true)
  .transparent(true)
  .skip_taskbar(true)
  .visible(false)
  .devtools(true)
  .build()
  .map_err(|e| format!("Failed to create window: {}", e))?;
  Ok(())
}




#[tauri::command]
pub fn update_drop_files(files: Vec<String>) -> Result<(), String> {
    let mut file_infos = Vec::new();
    for path_str in files {
        let is_dir = Path::new(&path_str).is_dir();
        file_infos.push(FileInfo {
            path: path_str,
            is_dir,
        });
    }
    let mut drop_files = DROP_FILES.lock().map_err(|e| e.to_string())?;
    *drop_files = file_infos;
    Ok(())
}

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
