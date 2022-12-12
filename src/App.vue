<template>
  <SearchBar @change="onSearchChange" />
  <ClipBoardList
    :select-index="selectIndex"
    :no-result="noResultFlag"
    :data="clipBoardDataList"
    @clickItem="clickDataItem"
    @changeIndex="changeIndex"
  />
  <KeyMapBar :key-map="keyMap" />
</template>
<script setup>
import SearchBar from "./components/SearchBar.vue";
import ClipBoardList from "./components/ClipBoardList.vue";
import KeyMapBar from "./components/KeyMapBar.vue";
import { appWindow, LogicalSize } from "@tauri-apps/api/window";
import { ref, onMounted, onBeforeMount, onUpdated, onUnmounted } from "vue";
import { selectPage, insertRecord, removeById } from "./service/recordService";
import { readText, writeText } from "@tauri-apps/api/clipboard";
import { listen, once } from "@tauri-apps/api/event";
import { message } from "@tauri-apps/api/dialog";
import { isRegistered, register, unregister } from "@tauri-apps/api/globalShortcut";
const mainShortCut = "CommandOrControl+Shift+C";
const noResultFlag = ref(false);
const selectIndex = ref(-1);
const lastClipBoardData = ref("");
const keyMap = [
  { keymap: ["Enter"], tips: "复制" },
  { keymap: ["Cmd+Nmb"], tips: "快捷复制" },
  { keymap: ["↑", "↓"], tips: "移动选择" },
  { keymap: ["Esc"], tips: "关闭" },
  { keymap: ["Cmd+Shift+C"], tips: "唤起" },
];
/**
 * @type {Array<{id: number, contentParse: Array<{content: string, match: boolean}>, contentSource: string}>}
 */
const clipBoardDataList = ref([]);
onBeforeMount(async () => {
  await initShortCut();
  await initClipBoardDataList();
});
let clipBoardListener;

onMounted(async () => {
  initAppShortCut();
  if (!clipBoardListener) {
    clipBoardListener = setInterval(async () => {
      let text = await readText();
      if (text === null) {
        return;
      }
      if (text.trim() === "") {
        return;
      }
      if (text !== lastClipBoardData.value) {
        lastClipBoardData.value = text;
        await insertRecord(text);
        await initClipBoardDataList();
      }
    }, 500);
  }
});

onUnmounted(async () => {
  if (clipBoardListener) {
    clearInterval(clipBoardListener);
  }
  await unRegisterShortCut();
});

onUpdated((e) => {
  let height = document.body.offsetHeight;
  let width = document.body.offsetWidth;
  appWindow.setSize(new LogicalSize(width, height));
});

const initClipBoardDataList = async () => {
  let res = await selectPage("");
  lastClipBoardData.value = res[0].content;
  clipBoardDataList.value = res.map((item) => formatData(item, ""));
};

const onSearchChange = async (value) => {
  if (value === "") {
    // reset flag
    noResultFlag.value = false;
  }
  // [{id: 1, content: "hello world"}]
  let res = await selectPage(value, 20);
  // format to clipBoardDataList
  clipBoardDataList.value = res.map((item) => formatData(item, value));
  if (res.length === 0) {
    noResultFlag.value = true;
  } else {
    noResultFlag.value = false;
  }
};

const changeIndex = (index) => {
  selectIndex.value = index;
};

const clickDataItem = async (index) => {
  console.log("clickDataItem", index);
  let item = clipBoardDataList.value[index];
  await writeText(item.contentSource);
  appWindow.hide();
};

const onKeyEnter = async () => {
  console.log("onKeyEnter");
  if (selectIndex.value === -1) {
    return;
  }
  let item = clipBoardDataList.value[selectIndex.value];
  await writeText(item.contentSource);
  appWindow.hide();
};

const formatData = (item, value) => {
  let contentSource = item.content;
  let contentParse = [];
  let matchIndex = contentSource.indexOf(value);
  if (matchIndex === -1) {
    contentParse.push({ content: contentSource, match: false });
  } else {
    let matchContent = contentSource.slice(matchIndex, matchIndex + value.length);
    let beforeContent = contentSource.slice(0, matchIndex);
    let afterContent = contentSource.slice(matchIndex + value.length);
    contentParse.push({ content: beforeContent, match: false });
    contentParse.push({ content: matchContent, match: true });
    contentParse.push({ content: afterContent, match: false });
  }
  return {
    id: item.id,
    contentParse,
    contentSource,
  };
};

const initShortCut = async () => {
  const isRegisteredFlag = await isRegistered(mainShortCut);
  if (isRegisteredFlag) {
    await message(`快捷键 ${mainShortCut} 被占用`, { title: "警告", type: "error" });
  } else {
    await register(mainShortCut, async () => {
      console.log("Shortcut triggered");
      let visible = await appWindow.isVisible();
      if (visible) {
        await appWindow.hide();
      } else {
        await appWindow.show();
        await appWindow.setFocus();
      }
    });
  }
};

const unRegisterShortCut = async () => {
  const isRegisteredFlag = await isRegistered(mainShortCut);
  if (isRegisteredFlag) {
    console.log("unRegisterShortCut");
    await unregister(mainShortCut);
  }
};

const moveIndex = (offset) => {
  let selectIndexTmp = selectIndex.value + offset;
  if (selectIndexTmp <= 0) {
    selectIndex.value = 0;
  } else if (selectIndexTmp >= clipBoardDataList.value.length - 1) {
    selectIndex.value = clipBoardDataList.value.length - 1;
  } else {
    selectIndex.value = selectIndexTmp;
  }
};

const initAppShortCut = async () => {
  document.onkeydown = async (e) => {
    let key = e.key;
    let isShift = e.shiftKey;
    let isMeta = e.metaKey;
    let isCtrl = e.ctrlKey;
    let isAlt = e.altKey;
    let isCmd = isMeta || isCtrl;
    let numberKey = ["1", "2", "3", "4", "5", "6", "7", "8", "9"];
    if (key == "Escape") {
      //esc
      await appWindow.hide();
    }
    if (key == "Up" || key == "ArrowUp") {
      //up
      moveIndex(-1);
    }
    if (key == "Down" || key == "ArrowDown") {
      //down
      moveIndex(1);
    }
    if (key == "Enter") {
      //enter
      await onKeyEnter();
    }
    if (isCmd && numberKey.includes(key)) {
      //cmd + 1
      await clickDataItem(parseInt(key) - 1);
    }
  };
};
</script>

<style scoped></style>
