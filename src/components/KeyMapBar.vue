<template>
  <footer class="key-map-bar" data-tauri-drag-region>
    <HotKeyItem
      :key="item.tips"
      v-for="item in keyMap"
      :keymap="item.keymap"
      :tips="item.tips"
    ></HotKeyItem>
  </footer>
</template>
<script setup>
import HotKeyItem from "./HotKeyItem.vue";
const keyMap = [
  { keymap: ["Enter"], tips: "搜索" },
  { keymap: ["↑", "↓"], tips: "移动选择" },
  { keymap: ["Esc"], tips: "关闭" },
];
</script>
<style scoped>
.key-map-bar {
  height: 44px;
  align-items: center;
  border-radius: 0 0 8px 8px;
  display: flex;
  flex-shrink: 0;
  align-items: center;
  padding: 0 12px;
  position: relative;
  user-select: none;
  background: #ffffff;
}
</style>
