<script>
import { ref } from "vue";
import { Window } from "@tauri-apps/api/window";

export default {
    components: {},
    setup() {
        // const xxEl = ref(null);
        // const appWindow = new Window("main");
        const appWindow = Window.getCurrent();
        // https://juejin.cn/post/7433822127390556196
        const minimize = () => {
            appWindow.minimize();
        };
        const maximize = () => {
            appWindow.toggleMaximize();
        };
        const close = () => {
            // appWindow.destroy()同样会关闭程序
            // 对于有系统托盘（下一篇文章会讲）的程序而言点击关闭按钮并不是想退出程序，而是关闭窗口保留托盘
            // 这时候需要使用appWindow.hide()隐藏主窗口，但是程序并没有被关闭。
            // 如果要显示主窗口可以在托盘事件中添加appWindow.show()代码，点击托盘可以再显示主窗口
            appWindow.close();
        };

        return { minimize, maximize, close };
    },
};
</script>

<template lang="pug">
.titlebar
    .title_label(data-tauri-drag-region) 自定义标题栏
    .titlebar-button(@click="minimize")
        img(src="https://api.iconify.design/mdi:window-minimize.svg" alt="minimize")
    .titlebar-button(@click="maximize")
        img(src="https://api.iconify.design/mdi:window-maximize.svg" alt="maximize")
    .titlebar-button(@click="close")
        img(src="https://api.iconify.design/mdi:close.svg" alt="close")
</template>

<style lang="scss">
.titlebar {
    height: 30px;
    background: #4dabf7;
    user-select: none;
    display: flex;
    justify-content: flex-end;
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
}
.title_label{
    width: 100%;
    height: 100%;
    display: flex;
    justify-content: center;
    align-items: center;
    color: #f0f0f0;
}

.titlebar-button {
    display: inline-flex;
    justify-content: center;
    align-items: center;
    width: 30px;
    height: 30px;
    user-select: none;
    -webkit-user-select: none;
}

.titlebar-button:hover {
    background: #f0f0f0;
}
</style>
