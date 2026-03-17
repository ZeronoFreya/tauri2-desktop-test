<script setup>
import { ref, onMounted, onUnmounted } from "vue";
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";


import Title from "@c/title.vue";
import DropApp from "@c/drop_app.vue";
import DropDom from "@c/drop_dom.vue";
import InputFileDialog from "@c/dialog.vue";

import useGlobalDrag from '@c/useGlobalDrag'

useGlobalDrag()

const greetMsg = ref("");
const name = ref("");



async function greet() {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    greetMsg.value = await invoke("greet", { name: name.value });
}

let unlisten = null;
onMounted(async () => {
    invoke("create_drop_window");

    unlisten = await listen("before-close", (event) => {
        // 监听到窗口关闭事件, 在此处决定是否允许窗口关闭
        invoke("force_close");
    });
});

onUnmounted(() => {
    if (unlisten) {
        unlisten();
    }
});

</script>
<template lang="pug">
main.container
    Title
    .drop-zone-group
        DropApp
        DropDom
    InputFileDialog
        
</template>

<style lang="scss">
:where(.container) {
    margin: 0;
    padding-top: 10vh;
    display: flex;
    flex-direction: column;
    justify-content: center;
    text-align: center;

    .drop-zone-group {
        display: flex;
        align-items: flex-start;
        justify-content: center;
        gap: 10px;
    }
}
</style>
