<template>
  <footer class="key-map-bar" data-tauri-drag-region>
    <HotKeyItem
      :key="item.tips"
      v-for="item in keyMap"
      :keymap="item.keymap"
      :tips="item.tips"
    ></HotKeyItem>
    <img src="../assets/gh-desktop.png" class="github-repository" @click="gotoGithub" />
  </footer>
</template>
<script setup>
import HotKeyItem from "./HotKeyItem.vue";
import { open } from "@tauri-apps/api/shell";
defineProps({
  keyMap: Array[Object],
});
const gotoGithub = () => {
  open("https://github.com/ChurchTao/Lanaya");
};
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
  box-shadow: 0 -1px 0 0 #e0e3e8, 0 -3px 6px 0 rgba(69, 98, 155, 0.12);
}
.github-repository {
  position: absolute;
  width: 28px;
  height: 28px;
  right: 16px;
  cursor: pointer;
}
</style>
