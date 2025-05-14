<script>
import { ref, onMounted, onBeforeMount } from "vue";
// import { Window } from "@tauri-apps/api/window";
// import { listen } from "@tauri-apps/api/event";

// import { WebviewWindow } from "@tauri-apps/api/webviewWindow";
import { invoke } from "@tauri-apps/api/core";
import DropAppOut from "./drop_app_out.vue";

import useGlobalDrag from '@c/useGlobalDrag'

export default {
    components: { DropAppOut },
    setup() {
        const dropPath = ref("");

        const { isDragDom } = useGlobalDrag()

        const handleDragEnter = (e) => {
            // 只有当事件发生在目标元素本身时才处理
            if (!isDragDom.value && !e.currentTarget.contains(e.relatedTarget)) {
                e.currentTarget.classList.add("dragover");
            }
        };

        const handleDragLeave = (e) => {
            if (!isDragDom.value &&!e.currentTarget.contains(e.relatedTarget)) {
                e.currentTarget.classList.remove("dragover");
            }
        };
        
        // 处理文件拖拽
        const handleDrop = async (e) => {
            e.preventDefault();
            e.currentTarget.classList.remove("dragover");
            if (isDragDom.value) {
                return;
            }
            // 获取拖拽文件
            let files = await invoke("take_drop_files");
            console.log("dropPath", files);
            // files : [
            //     {
            //         "path": "D:\\Download\\小书痴\\第5部「女神の化身4」",
            //         "is_dir": true
            //     },
            //     {
            //         "path": "D:\\Download\\小书痴\\本好きの下剋上朗読.mp4",
            //         "is_dir": false
            //     }
            // ]
            if (files.length > 0) {
                dropPath.value = files.map((file) => file.path).join(",");
            } else {
                dropPath.value = "";
            }
        };


        return {
            dropPath,
            handleDragEnter,
            handleDragLeave,
            handleDrop,
        };
    },
};
</script>

<template lang="pug">
.drop-demo
    .drop-zone(
        @dragenter.prevent="handleDragEnter" 
        @dragleave.prevent="handleDragLeave" 
        @drop.prevent="handleDrop") 
        span(v-if="dropPath") {{dropPath}}
        span(v-else) 将文件拖拽到这里
    .drag-box
        DropAppOut
</template>

<style lang="scss">
.drop-demo {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: flex-start;
    gap: 10px;
}
.drop-zone {
    width: 300px;
    height: 200px;
    border: 2px dashed #ccc;
    display: flex;
    align-items: center;
    justify-content: center;
    margin: 20px;
    transition: all 0.3s ease;
    user-select: none;
}

.drop-zone.dragover {
    background-color: rgba(50, 158, 163, 0.1);
    border-color: #329ea3;
    transform: scale(1.02);
}

.drag-box {
    display: flex;
    gap: 10px;
    padding: 10px;

    > div {
        cursor: grab;
    }
}
</style>
