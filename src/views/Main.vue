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
import Layout from "../components/Layout.vue";
import SearchBar from "../components/SearchBar.vue";
import ClipBoardList from "../components/ClipBoardList.vue";
import KeyMapBar from "../components/KeyMapBar.vue";
import { appWindow } from "@tauri-apps/api/window";
import { ref, onMounted, onBeforeMount, onUnmounted, nextTick } from "vue";
import { selectPage, insertRecord, removeById, clearAll } from "../service/recordService";
import { readText, writeText } from "@tauri-apps/api/clipboard";
import { listen } from "@tauri-apps/api/event";
import { message } from "@tauri-apps/api/dialog";
import { isRegistered, register, unregister } from "@tauri-apps/api/globalShortcut";
import { useI18n } from "vue-i18n";
import { registerCommonConfigConsumer } from "../service/globalListener";
const mainShortCut = "CommandOrControl+Shift+C";
const noResultFlag = ref(false);
const selectIndex = ref(-1);
const lastClipBoardData = ref("");
const cmdPressDown = ref(false);
const { t } = useI18n({
  inheritLocale: true,
  useScope: "global",
});
const keyMap = ref([]);
/**
 * @type {Array<{id: number, contentParse: Array<{content: string, match: boolean}>, contentSource: string}>}
 */
const clipBoardDataList = ref([]);
onBeforeMount(async () => {
  await initShortCut();
  await initListenr();
  await initClipBoardDataList();
});
let clipBoardListener;
let unlistenBlur;

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
  unlistenBlur();
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

const initKeyMapShow = () => {
  keyMap.value = [
    { keymap: ["???"], tips: t("hotkeys.copy") },
    { keymap: ["???+Nmb"], tips: t("hotkeys.quick-copy") },
    { keymap: ["???", "???"], tips: t("hotkeys.move-selected") },
    { keymap: ["Esc"], tips: t("hotkeys.close-window") },
    { keymap: ["?????????"], tips: t("hotkeys.clear-history") },
    { keymap: ["??????C"], tips: t("hotkeys.global-shortcut") },
  ];
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
    await message(`????????? ${mainShortCut} ?????????`, { title: "??????", type: "error" });
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

const initListenr = async () => {
  if (!unlistenBlur) {
    unlistenBlur = await listen("tauri://blur", async (event) => {
      console.log(`tauri://blur`, event);
      let visible = await appWindow.isVisible();
      if (visible) {
        await appWindow.hide();
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
const setDataItemAlwaysShow = (offset) => {
  const dataSelect = document.querySelector(".data-select")?.getBoundingClientRect();
  const dataList = document.querySelector(".data-list")?.getBoundingClientRect();
  if (dataSelect && dataList) {
    const dataSelectTop = dataSelect.top - dataList.top;
    const dataSelectBottom = dataSelectTop + dataSelect.height;
    const dataListHeight = dataList.height;
    if (dataSelectTop < 0 || dataSelectBottom > dataListHeight) {
      document.querySelector(".data-select")?.scrollIntoView({
        behavior: "smooth",
        block: offset < 0 ? "end" : "start",
        inline: "nearest",
      });
    }
  }
};
const moveIndex = (offset) => {
  let prevIndex = selectIndex.value;
  let selectIndexTmp = selectIndex.value + offset;
  if (selectIndexTmp <= 0) {
    selectIndex.value = 0;
  } else if (selectIndexTmp >= clipBoardDataList.value.length - 1) {
    selectIndex.value = clipBoardDataList.value.length - 1;
  } else {
    selectIndex.value = selectIndexTmp;
  }
  if (prevIndex !== selectIndex.value) {
    nextTick(() => {
      setDataItemAlwaysShow(offset);
    });
  }
};

const initAppShortCut = async () => {
  initKeyMapShow();
  registerCommonConfigConsumer((config) => {
    initKeyMapShow();
  });
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
