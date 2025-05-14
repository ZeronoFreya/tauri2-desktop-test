<script>
import { ref } from "vue";
import { startDrag } from "@crabnebula/tauri-plugin-drag";
import useGlobalDrag from '@c/useGlobalDrag'

export default {
    components: {},
    setup() {
        const { asDragDom } = useGlobalDrag()

        const filePath = ref("c:\\vue.svg");
        const handleDrag = async (e) => {
            e.preventDefault();
            e.stopPropagation();
            if (!filePath.value) return;
            try {
                asDragDom()
                await startDrag({
                    item: [filePath.value],
                    icon: "",
                    mode: "copy",
                });
            } catch (error) {
                console.error("拖拽失败:", error);
            }
        };

        return {
            handleDrag,
        };
    },
};
</script>

<template lang="pug">
.drag-handle(@mousedown.prevent="handleDrag") 拖拽到外部程序      
</template>

<style lang="scss">
.drag-handle {
    background-color: #4dabf7;
    padding: 30px;
    border-radius: 12px;
    color: #fff;
}
</style>
