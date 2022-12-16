<template>
  <Layout>
    <SearchBar @change="onSearchChange" />
    <ClipBoardList
      :select-index="selectIndex"
      :no-result="noResultFlag"
      :data="clipBoardDataList"
      :cmd-press-down="cmdPressDown"
      @clickItem="clickDataItem"
      @changeIndex="changeIndex"
    />
    <KeyMapBar :key-map="keyMap" />
  </Layout>
</template>
<script setup>
import Layout from "./components/Layout.vue";
import SearchBar from "./components/SearchBar.vue";
import ClipBoardList from "./components/ClipBoardList.vue";
import KeyMapBar from "./components/KeyMapBar.vue";
import { appWindow, LogicalSize } from "@tauri-apps/api/window";
import { ref, onMounted, onBeforeMount, onUnmounted } from "vue";
import { selectPage, insertRecord, removeById, clearAll } from "./service/recordService";
import { readText, writeText } from "@tauri-apps/api/clipboard";
import { message } from "@tauri-apps/api/dialog";
import { isRegistered, register, unregister } from "@tauri-apps/api/globalShortcut";
const mainShortCut = "CommandOrControl+Shift+C";
const noResultFlag = ref(false);
const selectIndex = ref(-1);
const lastClipBoardData = ref("");
const cmdPressDown = ref(false);
const keyMap = [
  { keymap: ["⏎"], tips: "复制" },
  { keymap: ["⌘+Nmb"], tips: "快捷复制" },
  { keymap: ["↑", "↓"], tips: "移动选择" },
  { keymap: ["Esc"], tips: "关闭" },
  { keymap: ["⌘⇧⌫"], tips: "清空历史" },
  { keymap: ["⌘⇧C"], tips: "全局唤起" },
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

const initClipBoardDataList = async () => {
  let res = await selectPage("");
  if (res) {
    if (res.length > 0) {
      lastClipBoardData.value = res[0].content;
    }
    clipBoardDataList.value = res.map((item) => formatData(item, ""));
  }
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
    selectIndex.value = -1;
  } else {
    noResultFlag.value = false;
    selectIndex.value = 0;
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

const onKeyBackspace = async () => {
  console.log("onKeyBackspace");
  if (selectIndex.value === -1) {
    return;
  }
  let item = clipBoardDataList.value[selectIndex.value];
  await removeById(item.id);
  await initClipBoardDataList();
};

const onClearAll = async () => {
  await clearAll();
  await initClipBoardDataList();
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
  // const unlistenDestroyed = await listen("tauri://destroyed", (event) => {
  //   console.log(`destroyed window`, event);
  // });
  // const unlistenCreated = await listen("tauri://window-created", (event) => {
  //   console.log(`created window`, event);
  // });
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
    if (isCmd) {
      cmdPressDown.value = true;
    }
    if (key == "Escape") {
      //esc
      await appWindow.hide();
      return;
    }
    if (key == "Up" || key == "ArrowUp") {
      //up
      moveIndex(-1);
      return;
    }
    if (key == "Down" || key == "ArrowDown") {
      //down
      moveIndex(1);
      return;
    }
    if (key == "Enter") {
      //enter
      await onKeyEnter();
      return;
    }
    if (isCmd && numberKey.includes(key)) {
      //cmd + 1
      await clickDataItem(parseInt(key) - 1);
      return;
    }
    if (isCmd && isShift && key == "Backspace") {
      await onClearAll();
      return;
    }
    // if (isCmd && key == "Backspace") {
    //   //cmd + backspace
    //   await onKeyBackspace();
    //   return;
    // }
  };

  document.onkeyup = async (e) => {
    let key = e.key;
    let isCmd = key == "Meta" || key == "Control";
    if (isCmd) {
      cmdPressDown.value = false;
    }
  };
};
</script>

<style scoped></style>
