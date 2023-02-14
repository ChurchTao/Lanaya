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
      @delete="deleteItem"
    />
    <KeyMapBar :key-map="keyMap" />
  </Layout>
</template>
<script setup>
import Layout from "@/components/Layout.vue";
import SearchBar from "@/components/SearchBar.vue";
import ClipBoardList from "@/components/ClipBoardList.vue";
import KeyMapBar from "@/components/KeyMapBar.vue";
import { ref, onMounted, onBeforeMount, onUnmounted, nextTick } from "vue";
import { selectPage, clearAll } from "@/service/recordService";
import { listen } from "@tauri-apps/api/event";
import { getShortCutShowAnyway, isDiff } from "@/service/shortCutUtil";
import { defaultHotkeys, hotkeys_func_enum } from "../config/constants";
import { closeWindowLater } from "@/service/windowUtil";
import {
  listenRecordLimitChange,
  listenHotkeysChange,
  listenClipboardChange,
} from "@/service/globalListener";
import { getCommonConfig, writeToClip } from "../service/cmds";
import hotkeys from "hotkeys-js";
const noResultFlag = ref(false);
const selectIndex = ref(-1);
const cmdPressDown = ref(false);
const keyMap = ref([]);
let unlistenBlur;
let unlistenRecordLimitChange;
let unlistenHotkeysChange;
let unlistenClipboardChange;
let recordLimit = 300;
let lastClipBoardData = "";
/**
 * @type {Array<{id: number, content: string, content_highlight: string}>}
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
});

onUnmounted(async () => {
  if (unlistenBlur) {
    unlistenBlur();
  }
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
  let res = await selectPage("", undefined, recordLimit);
  if (res) {
    clipBoardDataList.value = res.map((item) => formatData(item));
  }
};

const initKeyMapShow = (allKeys) => {
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
    noResultFlag.value = false;
  }
  let res = await selectPage(value, undefined, 20);
  clipBoardDataList.value = res.map((item) => formatData(item));
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
  let item = clipBoardDataList.value[index];
  await writeToClip(item.id);
  closeWindowLater(3000);
};

const deleteItem = async (index) => {
  clipBoardDataList.value.splice(index, 1);
};

const onKeyEnter = async () => {
  if (selectIndex.value === -1) {
    return;
  }
  let item = clipBoardDataList.value[selectIndex.value];
  await writeToClip(item.id);
  closeWindowLater(3000);
};

const onClearAll = async () => {
  await clearAll();
  await initClipBoardDataList();
};

const formatData = (item) => {
  return {
    id: item.id,
    content: item.content,
    content_highlight: item.content_highlight,
    type: item.data_type,
    is_favorite: item.is_favorite,
  };
};

const refreshShortCut = () => {
  let allKeys = [...defaultHotkeys, ...JSON.parse(JSON.stringify(shortCuts.value))];
  let appShortCuts = allKeys.filter((item) => {
    return !item.func.startsWith("global");
  });
  initKeyMapShow(allKeys);
  initAppShortCut(appShortCuts);
};

const initListenr = async () => {
  if (!unlistenBlur) {
    unlistenBlur = await listen("tauri://blur", async (event) => {
      closeWindowLater(3000);
    });
  }
  if (!unlistenClipboardChange) {
    unlistenClipboardChange = await listenClipboardChange(async (event) => {
      await initClipBoardDataList();
    });
  }
  if (!unlistenRecordLimitChange) {
    unlistenRecordLimitChange = await listenRecordLimitChange((newLimitNum) => {
      recordLimit = newLimitNum;
    });
  }
  if (!unlistenHotkeysChange) {
    unlistenHotkeysChange = await listenHotkeysChange((hotkeys) => {
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
            case hotkeys_func_enum.CLEAR_HISTORY:
              onClearAll();
              break;
            case hotkeys_func_enum.COPY:
              onKeyEnter();
              break;
            case hotkeys_func_enum.CLOSE_WINDOW:
              closeWindowLater(3000);
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
