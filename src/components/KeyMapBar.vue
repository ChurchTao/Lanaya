<template>
  <div
    class="key-map-bar h-12 text-center flex items-center flex-shrink-0 py-0 px-4 relative select-none bg-white rounded-b-xl"
    data-tauri-drag-region
  >
    <HotKeyItem
      v-for="item in keyMap"
      :key="item.tips"
      :show="hotKeyShow(item.keymap)"
      :keymap="item.keymap"
      :tips="item.tips"
    ></HotKeyItem>
    <img
      src="../assets/gh-desktop.png"
      class="absolute w-8 h-8 right-4 cursor-pointer"
      @click="gotoGithub"
    />
  </div>
</template>
<script setup>
import HotKeyItem from "./HotKeyItem.vue";
import { open } from "@tauri-apps/api/shell";
// import { WebviewWindow } from "@tauri-apps/api/window";
defineProps({
  keyMap: Array[Object],
});
const gotoGithub = async () => {
  open("https://github.com/ChurchTao/Lanaya");
  // const webview = new WebviewWindow("config", {
  // url: "/config",
  // });
};
const hotKeyShow = (keymap) => {
  return keymap.length > 0 && keymap[0] !== "";
};
</script>
<style scoped>
.key-map-bar {
  box-shadow: 0 -1px 0 0 #e0e3e8, 0 -3px 6px 0 rgba(69, 98, 155, 0.12);
}
</style>
