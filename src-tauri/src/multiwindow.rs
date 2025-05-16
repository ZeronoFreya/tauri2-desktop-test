use serde::Deserialize;
use std::sync::atomic::{AtomicUsize, Ordering};
use tauri::{Manager, PhysicalPosition, WindowEvent};

static WINDOW_COUNTER: AtomicUsize = AtomicUsize::new(0);

/// 窗口的配置参数
#[derive(Deserialize)]
pub struct WindowConfig {
    url: String,
    size: Option<WindowSize>,
    relative_pos: Option<RelativePosition>,
}

#[derive(Deserialize)]
pub struct WindowSize {
    width: f64,
    height: f64,
}

#[derive(Deserialize)]
pub struct RelativePosition {
    x: i32,
    y: i32,
}

#[derive(Deserialize)]
pub struct WindowUpdate {
    size: Option<WindowSize>,
    relative_pos: Option<RelativePosition>,
}

/// 窗口跟踪器，用于管理父子窗口关系
pub struct WindowTracker {
    label: String,
    child: tauri::WebviewWindow,
    parent: tauri::WebviewWindow,
    offset: (i32, i32),
    config: WindowConfig,
}

impl WindowTracker {
    /// 创建新的窗口跟踪器
    fn new(app: &tauri::AppHandle, config: WindowConfig) -> Result<Self, String> {
        // 获取主窗口
        let parent = app
            .get_webview_window("main")
            .ok_or("Main window not found")?;

        // 计算初始偏移
        let parent_inner_pos = parent.inner_position()
            .map_err(|e| format!("Failed to get parent inner position: {}", e))?;
        let parent_outer_pos = parent.outer_position()
            .map_err(|e| format!("Failed to get parent outer position: {}", e))?;

        let offset = (
            parent_inner_pos.x - parent_outer_pos.x,
            parent_inner_pos.y - parent_outer_pos.y,
        );

        // 计算窗口位置
        let window_pos = if let Some(rel_pos) = config.relative_pos {
            PhysicalPosition {
                x: parent_inner_pos.x + rel_pos.x,
                y: parent_inner_pos.y + rel_pos.y,
            }
        } else {
            parent_inner_pos
        };

        // 生成唯一标签
        let label = format!("tab-window-{}", WINDOW_COUNTER.fetch_add(1, Ordering::SeqCst));

        // 创建窗口构建器
        let mut builder = tauri::WebviewWindowBuilder::new(
            app,
            &label,
            tauri::WebviewUrl::External(config.url.parse().map_err(|e| format!("Invalid URL: {}", e))?)
        );

        // 设置窗口尺寸
        if let Some(size) = &config.size {
            builder = builder.inner_size(size.width, size.height);
        } else {
            builder = builder.inner_size(400.0, 300.0);
        }

        // 构建子窗口
        let child = builder
            .position(window_pos.x as f64, window_pos.y as f64)
            .parent(&parent)
            .map_err(|e| format!("Failed to set parent window: {}", e))?
            .decorations(false)
            .shadow(false)
            .build()
            .map_err(|e| format!("Failed to build child window: {}", e))?;

        Ok(Self {
            label,
            child,
            parent,
            offset,
            config,
        })
    }

    /// 获取窗口标签
    pub fn label(&self) -> &str {
        &self.label
    }

    /// 更新窗口位置
    pub fn update_position(&self, parent_pos: PhysicalPosition<i32>) {
        let new_pos = PhysicalPosition {
            x: parent_pos.x + self.offset.0,
            y: parent_pos.y + self.offset.1,
        };
        let _ = self.child.set_position(new_pos);
    }

    /// 隐藏窗口
    pub fn hide(&self) -> Result<(), String> {
        self.child.hide()
            .map_err(|e| format!("Failed to hide window {}: {}", self.label, e))
    }

    /// 显示窗口
    pub fn show(&self) -> Result<(), String> {
        self.child.show()
            .map_err(|e| format!("Failed to show window {}: {}", self.label, e))
    }

    /// 关闭窗口
    pub fn close(&self) -> Result<(), String> {
        self.child.close()
            .map_err(|e| format!("Failed to close window {}: {}", self.label, e))
    }

    /// 更新窗口的尺寸和位置
    pub fn update_config(&mut self, update: WindowUpdate) -> Result<(), String> {
        // 更新尺寸
        if let Some(size) = &update.size {
            self.child.set_size(tauri::PhysicalSize::new(
                size.width as u32,
                size.height as u32,
            ))
            .map_err(|e| format!("Failed to update window size: {}", e))?;
        }

        // 更新相对位置
        if let Some(rel_pos) = &update.relative_pos {
            let parent_pos = self.parent.inner_position()
                .map_err(|e| format!("Failed to get parent position: {}", e))?;
            
            let new_pos = PhysicalPosition {
                x: parent_pos.x + rel_pos.x,
                y: parent_pos.y + rel_pos.y,
            };
            
            self.child.set_position(new_pos)
                .map_err(|e| format!("Failed to update window position: {}", e))?;
        }

        Ok(())
    }
}

/// 创建一个子窗口并设置与父窗口的关联
#[tauri::command]
pub fn create_tab_window(app: tauri::AppHandle, config: WindowConfig) -> Result<String, String> {
    let tracker = WindowTracker::new(&app, config)?;
    let label = tracker.label().to_string();

    // 设置父窗口移动事件监听
    setup_window_events(&tracker.parent, app.clone());

    // 存储跟踪器
    app.manage(tracker);

    Ok(label)
}

/// 设置窗口事件监听
fn setup_window_events(parent: &tauri::WebviewWindow, app_handle: tauri::AppHandle) {
    parent.on_window_event(move |event| {
        if let WindowEvent::Moved(pos) = event {
            if let Some(tracker) = app_handle.try_state::<WindowTracker>() {
                tracker.update_position(pos);
            }
        }
    });
}


/// 隐藏指定标签的窗口
#[tauri::command]
pub fn hide_tab_window(app: tauri::AppHandle, label: &str) -> Result<(), String> {
    if let Some(window) = app.get_webview_window(label) {
        window.hide()
            .map_err(|e| format!("Failed to hide window {}: {}", label, e))?;
    }
    Ok(())
}

/// 显示指定标签的窗口
#[tauri::command]
pub fn show_tab_window(app: tauri::AppHandle, label: &str) -> Result<(), String> {
    if let Some(window) = app.get_webview_window(label) {
        window.show()
            .map_err(|e| format!("Failed to show window {}: {}", label, e))?;
    }
    Ok(())
}

/// 关闭指定标签的窗口
#[tauri::command]
pub fn close_tab_window(app: tauri::AppHandle, label: &str) -> Result<(), String> {
    if let Some(window) = app.get_webview_window(label) {
        window.close()
            .map_err(|e| format!("Failed to close window {}: {}", label, e))?;
    }
    Ok(())
}

/// 更新窗口配置
#[tauri::command]
pub fn update_tab_window(
    app: tauri::AppHandle,
    label: &str,
    update: WindowUpdate,
) -> Result<(), String> {
    // 获取窗口跟踪器的可变引用
    let state: tauri::State<WindowTracker> = app.state();
    let mut tracker = state.inner();
    
    if tracker.label() == label {
        tracker.update_config(update)
    } else {
        Err(format!("Window with label '{}' not found", label))
    }
}