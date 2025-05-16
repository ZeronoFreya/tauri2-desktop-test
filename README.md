# tauri2_desktop_test
测试 v2 作为桌面软件是否合格

![程序界面](readme/gui.png)

---


Tauri2如果要获取拖拽文件的`真实路径`，必须开启`dragDropEnabled`，原理是接管拖拽事件，`副作用`是html的拖拽失效，也就是说拖拽插件会阵亡。

若不使用dragDropEnabled，会受限于浏览器的安全策略，只能获取文件内容，如下：
```json
 {
    "name": "example.jpg",
    "size": 1024,
    "type": "image/jpeg",
    "lastModified": 1625097600000,
    "lastModifiedDate": "2021-06-30T00:00:00.000Z",
    "webkitRelativePath": ""
  }
```

### 仅靠文件名、文件大小、编辑时间无法可靠锁定文件，所以无法使用

但为了获取文件路径就放弃html5的拖拽功能，实属捡了芝麻丢了西瓜——当然，这依程序功能与个人喜好而定。

综合考虑，我决定两者都要。
`实现原理`很简单，再开一个透明窗口用来接受拖拽事件即可。

---

## 打算重构为仅用rust实现拖拽捕获功能，以下内容随时可能更改

只解释必要的部分
* 前端
    * public\drop-window.html  透明窗口用到的页面
    * src\components\useGlobalDrag.js  标记拖拽是来自外部还是内部
        * 在App.vue注册，全局共享状态
        * 组建中导入不会重复注册
* 后端
    * src-tauri\tauri.conf.json 
        * `app\withGlobalTauri: true`  否则drop-window.html中不能使用 `window.__TAURI__`
        * `app\windows\dragDropEnabled: false`  启用HTML5拖拽
    * src-tauri\capabilities\default.json\permissions
        * `core:window:allow-start-dragging` 自定义窗口拖拽要用，比如标题栏
        * `drag:default`  tauri-plugin-drag 功能是拖拽文件到外部程序
        * `dialog:default`  tauri-plugin-dialog 功能是[打开|保存]文件或目录
        * `dialog:allow-open` 同上
    * src-tauri\capabilities\drop-window.json\permissions
        * `core:window:allow-hide`
    * src-tauri\src\drop.rs
        * `create_drop_window` 创建透明的子窗口，用于捕捉拖拽文件
        * `update_drop_files`  保存当前拖拽的文件列表
        * `take_drop_files`    获取并清理当前拖拽的文件列表
        * `hide_drop_window`
        * `show_drop_window`