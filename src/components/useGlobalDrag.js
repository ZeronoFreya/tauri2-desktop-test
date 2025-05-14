import { ref, computed, onMounted, onBeforeMount } from "vue";
import { invoke } from "@tauri-apps/api/core";

// 状态枚举
const DragState = {
  IDLE: "idle", // 空闲状态
  INTERNAL_DRAG: "internal", // 内部拖拽
  EXTERNAL_OVER: "external", // 外部拖入
};

const state = {
  dragStatus: ref(DragState.IDLE),
  eventBound: false,
};

export default function useGlobalDrag() {
  // 计算属性优化
  const isDragDom = computed(
    () => state.dragStatus.value === DragState.INTERNAL_DRAG
  );

  const setupListeners = () => {
    if (state.eventBound) return;

    const handleDragStart = (e) => {
      state.dragStatus.value = DragState.INTERNAL_DRAG;
    };

    const handleDragEnter = (e) => {
      e.preventDefault();
      if (state.dragStatus.value === DragState.IDLE) {
        state.dragStatus.value = DragState.EXTERNAL_OVER;
        invoke("show_drop_window");
      }
    };

    const handleDragEnd = (e) => {
      setTimeout(() => {
        state.dragStatus.value = DragState.IDLE;
      }, 500);

      invoke("hide_drop_window");
    };

    onMounted(() => {
      // 外部拖入的操作不会触发 dragstart
      // 内部拖拽元素一定会触发 dragstart
      // 需要使用 capture 捕获阶段触发，防止子元素阻断事件流
      document.addEventListener("dragstart", handleDragStart, {
        capture: true,
      });
      document.addEventListener("dragenter", handleDragEnter, {
        capture: true,
      });
      document.addEventListener("drop", handleDragEnd, {
        capture: true,
        passive: true, // 避免阻塞滚动
      });
      document.addEventListener("dragend", handleDragEnd, { capture: true });
      state.eventBound = true;
    });

    onBeforeMount(() => {
      document.removeEventListener("dragstart", handleDragStart, {
        capture: true,
      });
      document.removeEventListener("dragenter", handleDragEnter, {
        capture: true,
      });
      document.removeEventListener("drop", handleDragEnd, { capture: true });
      document.removeEventListener("dragend", handleDragEnd, { capture: true });
      state.eventBound = false;
    });
  };

  const asDragDom = () => {
    state.dragStatus.value = DragState.INTERNAL_DRAG;
  };

  if (typeof document !== "undefined") {
    setupListeners();
  }

  return {
    isDragDom,
    asDragDom,
  };
}
