<script>
import { ref } from "vue";

export default {
    components: {},
    setup() {
        const xxEl = ref(null);
        const handleColorDrag = (e) => {
            e.dataTransfer.setData("text/plain", e.target.dataset.color);
        };
        const handleColorDrop = (e) => {
            e.target.style.backgroundColor =
                e.dataTransfer.getData("text/plain") || "#ccc";
        };
        return { handleColorDrag, handleColorDrop };
    },
};
</script>

<template lang="pug">
.drop-demo
    .drop-zone(@drop.prevent="handleColorDrop" ) 将色块拖拽到这里
    .drag-box
        .color-block.red(draggable="true" data-color="#ff6b6b" @dragstart="handleColorDrag")
        .color-block.blue(draggable="true" data-color="#4dabf7" @dragstart="handleColorDrag")
        .color-block.green(draggable="true" data-color="#51cf66" @dragstart="handleColorDrag")    
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
.color-block {
    width: 50px;
    height: 50px;
    border-radius: 4px;
}

.red {
    background-color: #ff6b6b;
}

.blue {
    background-color: #4dabf7;
}

.green {
    background-color: #51cf66;
}
</style>
