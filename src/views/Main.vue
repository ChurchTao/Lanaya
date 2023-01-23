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
import Layout from "@/components/Layout.vue";
import SearchBar from "@/components/SearchBar.vue";
import ClipBoardList from "@/components/ClipBoardList.vue";
import KeyMapBar from "@/components/KeyMapBar.vue";
import { appWindow } from "@tauri-apps/api/window";
import { ref, onMounted, onBeforeMount, onUnmounted, nextTick } from "vue";
import { selectPage, insertRecord, removeById, clearAll } from "@/service/recordService";
import { readText, writeText } from "@tauri-apps/api/clipboard";
import { listen } from "@tauri-apps/api/event";
import { register, unregisterAll } from "@tauri-apps/api/globalShortcut";
import { getShortCutName, getShortCutShowAnyway, isDiff } from "@/service/shortCutUtil";
import { defaultHotkeys } from "../config/constants";
import { listenRecordLimitChange, listenHotkeysChange } from "@/service/globalListener";
import { getCommonConfig } from "../service/cmds";
import hotkeys from "hotkeys-js";
const noResultFlag = ref(false);
const selectIndex = ref(-1);
const lastClipBoardData = ref("");
const cmdPressDown = ref(false);
const keyMap = ref([]);
let clipBoardListener;
let unlistenBlur;
let unlistenRecordLimitChange;
let unlistenHotkeysChange;
let recordLimit = 300;
/**
 * @type {Array<{id: number, contentParse: Array<{content: string, match: boolean}>, contentSource: string}>}
 */
const clipBoardDataList = ref([]);
const shortCuts = ref([
  {
    func: "clear-history",
    keys: [],
  },
  {
    func: "global-shortcut",
    keys: [],
  },
]);
onBeforeMount(async () => {
  await initListenr();
  await initClipBoardDataList();
});

onMounted(() => {
  initCommonConfig().then(() => {
    refreshShortCut();
  });
  initClipBoardListener();
});

onUnmounted(async () => {
  if (clipBoardListener) {
    clearInterval(clipBoardListener);
  }
  unlistenBlur();
});

const initCommonConfig = async () => {
  let res = await getCommonConfig();
  if (res.record_limit) {
    recordLimit = res.record_limit;
  }
  if (res.hotkeys) {
    shortCuts.value.forEach((item) => {
      let find = res.hotkeys.find((hotkey) => {
        return hotkey.startsWith(item.func);
      });
      if (find) {
        let strArr = find.split(":")[1].split("+");
        if (strArr.length > 0 && strArr[0] !== "") {
          item.keys = strArr.map((item) => {
            return parseInt(item);
          });
        }
      }
    });
  }
};

const initClipBoardDataList = async () => {
  let res = await selectPage("", recordLimit);
  if (res) {
    if (res.length > 0) {
      lastClipBoardData.value = res[0].content;
    }
    clipBoardDataList.value = res.map((item) => formatData(item, ""));
  }
};

const initKeyMapShow = (allKeys) => {
  console.log("initKeyMapShow", allKeys);
  let keyShowArr = allKeys.map((item) => {
    let keyShow = getShortCutShowAnyway(item.keys);
    return {
      keymap: keyShow,
      tips: `hotkeys.${item.func}`,
    };
  });
  keyMap.value = keyShowArr;
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

const refreshShortCut = () => {
  let allKeys = [...defaultHotkeys, ...JSON.parse(JSON.stringify(shortCuts.value))];
  let globalShortCuts = allKeys.filter((item) => {
    return item.func.startsWith("global");
  });
  let appShortCuts = allKeys.filter((item) => {
    return !item.func.startsWith("global");
  });
  initKeyMapShow(allKeys);
  initGlobalShortCut(globalShortCuts);
  initAppShortCut(appShortCuts);
};

const initGlobalShortCut = async (globalShortCuts) => {
  await unregisterAll();
  for (let index = 0; index < globalShortCuts.length; index++) {
    const item = globalShortCuts[index];
    if (item.func === "global-shortcut") {
      if (item.keys.length > 0 && item.keys[0] != "") {
        let mainShortCut = getShortCutName(item.keys, true);
        await register(mainShortCut, async () => {
          let visible = await appWindow.isVisible();
          if (visible) {
            await appWindow.hide();
          } else {
            await appWindow.show();
            await appWindow.setFocus();
          }
        });
      }
    }
  }
};

const initListenr = async () => {
  if (!unlistenBlur) {
    unlistenBlur = await listen("tauri://blur", async (event) => {
      let visible = await appWindow.isVisible();
      if (visible) {
        await appWindow.hide();
      }
    });
  }
  if (!unlistenRecordLimitChange) {
    unlistenRecordLimitChange = await listenRecordLimitChange((newLimitNum) => {
      recordLimit = newLimitNum;
    });
  }
  if (!unlistenHotkeysChange) {
    unlistenHotkeysChange = await listenHotkeysChange((hotkeys) => {
      console.log("hotkeysChange", hotkeys);
      shortCuts.value.forEach((item) => {
        let find = hotkeys.find((hotkey) => {
          return hotkey.startsWith(item.func);
        });
        if (find) {
          let strArr = find.split(":")[1].split("+");
          if (strArr.length === 1 && strArr[0] == "") {
            item.keys = [];
          } else {
            item.keys = strArr.map((item) => {
              return parseInt(item);
            });
          }
        }
      });
      refreshShortCut();
    });
  }
};

const initClipBoardListener = () => {
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

const initAppShortCut = async (appShortCuts) => {
  console.log("initAppShortCut", appShortCuts);
  hotkeys.filter = function (event) {
    return true;
  };
  hotkeys(
    "up,down",
    {
      capture: true,
      scope: "main",
    },
    (event, handler) => {
      if (event.key === "ArrowUp") {
        moveIndex(-1);
      } else if (event.key === "ArrowDown") {
        moveIndex(1);
      }
    }
  );
  hotkeys(
    "*",
    {
      capture: true,
      scope: "main",
    },
    () => {
      appShortCuts.forEach((shortCut) => {
        if (!isDiff(shortCut.keys, hotkeys.getPressedKeyCodes())) {
          switch (shortCut.func) {
            case "clear-history":
              onClearAll();
              break;
            case "copy":
              onKeyEnter();
              break;
            case "close-window":
              appWindow.hide();
              break;
          }
        }
      });
    }
  );
  hotkeys.setScope("main");

  document.onkeydown = async (e) => {
    let key = e.key;
    let isMeta = e.metaKey;
    let isCtrl = e.ctrlKey;
    let isCmd = isMeta || isCtrl;
    let numberKey = ["1", "2", "3", "4", "5", "6", "7", "8", "9"];
    if (isCmd) {
      cmdPressDown.value = true;
    }
    if (isCmd && numberKey.includes(key)) {
      //cmd + 1
      await clickDataItem(parseInt(key) - 1);
      return;
    }
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
