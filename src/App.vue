<script setup>
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";


// import { listen, TauriEvent } from "@tauri-apps/api/event";
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

onMounted( () => {
    invoke("create_drop_window");
})  

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
