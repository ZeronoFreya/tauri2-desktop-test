<script>
import { ref, computed } from "vue";
import { open } from "@tauri-apps/plugin-dialog";

export default {
    components: {},
    setup() {
        const filePath = ref("");
        const multiple = ref(true)
        const directory = ref(false)
        const onlyDoc = ref(true)

        const filters = computed(()=>{
            if(onlyDoc.value){
                return [
                        {
                            name: "文档文件",
                            extensions: ["pdf", "doc", "docx", "txt", "md"],
                        }
                    ]
            }
            return [{ name: "所有文件", extensions: ["*"] }]
        })
        const openFileDialog = async () => {
            try {
                const file = await open({
                    multiple: multiple.value,   // 是否允许多选
                    directory: directory.value, // 是否只选择目录
                    filters: filters.value,
                });
                if (file) {
                    filePath.value = multiple.value ? file.join("\n") : file;
                }

            } catch (error) {
                console.error("打开文件选择器失败:", error);
            }
        };
        return {
            filePath,
            multiple,
            directory,
            onlyDoc,
            openFileDialog
        };
    },
};
</script>

<template lang="pug">
.input_file_dialog_box
    .input_configs
        .ipt_cfg
            input(type="checkbox" v-model="multiple")
            span &nbsp;多选
        .ipt_cfg(v-show="!directory")
            input(type="checkbox" v-model="onlyDoc")
            span &nbsp;仅文档类型
        .ipt_cfg
            input(type="checkbox" v-model="directory")
            span &nbsp;文件夹
    .input_file_dialog(@click="openFileDialog") 
        p(v-if="filePath") {{filePath}}
        span(v-else-if="directory") 打开文件夹
        span(v-else) 打开文件
</template>

<style lang="scss">
.input_file_dialog_box {
    margin-top: 30px;
    display: flex;
    justify-content: space-around;
}

.input_configs {
    display: flex;
    flex-direction: column;
    align-items: flex-start;
}

.ipt_cfg {
    padding: 5px;
    input{
        accent-color: #51cf66;
    }
}

.input_file_dialog {
    width: 60%;
    background-color: #51cf66;
    padding: 30px;
    border-radius: 12px;
    color: #fff;

    >p {
        white-space: pre-line;
    }
}
</style>
