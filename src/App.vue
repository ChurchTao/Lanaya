<template>
  <SearchBar />
  <ClipBoardList :no-result="noResultFlag" :data="clipBoardDataList" />
  <KeyMapBar />
</template>

<script setup>
import SearchBar from "./components/SearchBar.vue";
import ClipBoardList from "./components/ClipBoardList.vue";
import KeyMapBar from "./components/KeyMapBar.vue";
import { appWindow, LogicalSize } from "@tauri-apps/api/window";
import { ref } from "vue";

const noResultFlag = ref(false);
const clipBoardDataList = [
  {
    contentParse: [
      { content: "hel", match: true },
      { content: "lo world", match: false },
    ],
    contentSource: "hello world",
  },
  {
    contentParse: [
      { content: "hel", match: true },
      { content: "lo world2", match: false },
    ],
    contentSource: "hello world2",
  },
  {
    contentParse: [
      { content: "hel", match: true },
      { content: "lo world3", match: false },
    ],
    contentSource: "hello world3",
  },
  {
    contentParse: [
      { content: "hel", match: true },
      { content: "lo world", match: false },
    ],
    contentSource: "hello world",
  },
  {
    contentParse: [
      { content: "hel", match: true },
      { content: "lo world2", match: false },
    ],
    contentSource: "hello world2",
  },
  {
    contentParse: [
      { content: "hel", match: true },
      { content: "lo world3", match: false },
    ],
    contentSource: "hello world3",
  },
  {
    contentParse: [
      { content: "hel", match: true },
      { content: "lo world", match: false },
    ],
    contentSource: "hello world",
  },
  {
    contentParse: [
      { content: "hel", match: true },
      { content: "lo world2", match: false },
    ],
    contentSource: "hello world2",
  },
  {
    contentParse: [
      { content: "hel", match: true },
      { content: "lo world3", match: false },
    ],
    contentSource: "hello world3",
  },
];
setTimeout(() => {
  let height = document.body.offsetHeight;
  let width = document.body.offsetWidth;
  appWindow.setSize(new LogicalSize(width, height));
}, 500);
</script>

<style scoped></style>
