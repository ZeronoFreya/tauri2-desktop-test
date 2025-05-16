import { invoke } from "@tauri-apps/api/core";

// 更新窗口尺寸和位置
await invoke("update_tab_window", {
  label: "tab-window-0",
  update: {
    size: {
      width: 1024,
      height: 768
    },
    relative_pos: {
      x: 200,
      y: 100
    }
  }
});

// 只更新尺寸
await invoke("update_tab_window", {
  label: "tab-window-0",
  update: {
    size: {
      width: 1024,
      height: 768
    }
  }
});

// 只更新位置
await invoke("update_tab_window", {
  label: "tab-window-0",
  update: {
    relative_pos: {
      x: 200,
      y: 100
    }
  }
});