import { defineStore } from "pinia";
import { getCommonConfig, setCommonConfig } from "../service/cmds";
import {
  listenRecordLimitChange,
  listenAutoPasteChange,
  listenDeleteConfirmChange,
} from "@/service/globalListener";
import { defaultHotkeys } from "../config/constants";

export const globalData = defineStore("globalData", {
  state: () => {
    return {
      common_config: {
        language: "zh",
        theme_mode: "light",
        enable_auto_launch: false,
        enable_auto_paste: false,
        enable_delete_confirm: false,
        hotkeys: ["clear-history:8+18", "global-shortcut:18+32"],
        record_limit: 100,
      },
      short_cut: [],
    };
  },
  getters: {
    config(state) {
      return state.common_config;
    },
    short_cuts(state) {
      return state.short_cut;
    },
  },
  actions: {
    async initCommonConfig() {
      console.log("initCommonConfig");
      await this.refreshCommonConfig();
      await this.initListener();
    },
    async initListener() {
      await listenRecordLimitChange((newLimitNum) => {
        this.common_config.record_limit = newLimitNum;
      });
      await listenAutoPasteChange((value) => {
        this.common_config.enable_auto_paste = value;
      });
      await listenDeleteConfirmChange((value) => {
        console.log("listenDeleteConfirmChange", value);
        this.common_config.enable_delete_confirm = value;
      });
    },
    async refreshCommonConfig() {
      let res = await getCommonConfig();
      this.common_config = res;
      if (res.hotkeys) {
        await this.patchShotCuts(res.hotkeys);
      }
    },
    async patchShotCuts(hotkeys) {
      let short_cuts = [];
      defaultHotkeys.forEach((item) => {
        let find = hotkeys.find((hotkey) => {
          return hotkey.startsWith(item.func);
        });
        if (find) {
          let strArr = find.split(":")[1].split("+");
          if (strArr.length > 0 && strArr[0] !== "") {
            let keys = strArr.map((item) => {
              return parseInt(item);
            });
            short_cuts.push({
              func: item.func,
              keys,
            });
          }
        } else {
          short_cuts.push(item);
        }
      });
      this.short_cut = short_cuts;
    },
    // 持久化
    async saveCommonConfig() {
      await setCommonConfig(this.common_config);
    },
  },
});

// modify like this:
// store.$patch((state) => {
//   state.common_config.language = 'en'
// })
