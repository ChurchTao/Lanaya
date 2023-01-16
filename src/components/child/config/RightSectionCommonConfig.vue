<template>
  <div class="common-config-container px-4 py-4">
    <div class="check-config-item flex">
      <div class="check-config-item-value">
        <input type="checkbox" v-model="commonConfig.enable_auto_launch" @change="change" />
      </div>
      <div class="check-config-item-name">
        {{ t("config.common.enable_auto_launch") }}
      </div>
    </div>
    <div class="select-config-item flex">
      <div class="select-config-item-name">
        {{ t("config.common.language") }}
      </div>
      <div class="select-config-item-value">
        <select v-model="commonConfig.language" @change="change">
          <option value="zh">简体中文</option>
          <option value="en">English</option>
        </select>
      </div>
    </div>
    <div class="select-config-item flex">
      <div class="select-config-item-name">
        {{ t("config.common.theme_mode") }}
      </div>
      <div class="select-config-item-value">
        <select v-model="commonConfig.theme_mode" @change="change">
          <option value="light">Light</option>
          <option value="dark">Dark</option>
        </select>
      </div>
    </div>
    <div class="select-config-item flex">
      <div class="select-config-item-name">
        {{ t("config.common.record_limit") }}
      </div>
      <div class="select-config-item-value">
        <select v-model="commonConfig.record_limit" @change="change">
          <option :value="50">50</option>
          <option :value="100">100</option>
          <option :value="200">200</option>
          <option :value="300">300</option>
        </select>
      </div>
    </div>
    <div class="select-config-item">
      <div class="select-config-item-name">
        {{ t("config.common.hotkeys") }}
      </div>
      <div class="select-config-item-list">
        <HotKeyInput
          v-for="item in shortCuts"
          :key="item.func"
          :func="item.func"
          :keys="item.keys"
          @change="shortCutChange"
        ></HotKeyInput>
      </div>
    </div>
  </div>
</template>

<script>
export default {
  name: "RightSectionCommonConfig",
};
</script>
<script setup>
import { getCommonConfig, setCommonConfig } from "@/service/cmds";
import { useI18n } from "vue-i18n";
import { ref, onMounted } from "vue";
import HotKeyInput from "@/components/child/config/HotKeyInput.vue";
const { t } = useI18n({
  inheritLocale: true,
  useScope: "global",
});
// enable_auto_launch: false
// language: "zh"
// record_limit: 300
// theme_mode: "light"
// hotkeys: null
const commonConfig = ref({});
// []
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

const getCommonConfigFromService = async () => {
  const res = await getCommonConfig();
  commonConfig.value = res;
  if (res.hotkeys) {
    shortCuts.value.forEach((item) => {
      let find = res.hotkeys.find((hotkey) => {
        return hotkey.startsWith(item.func);
      });
      if (find) {
        item.keys = find.split(":")[1].split("+");
      }
    });
  }
};

const setCommonConfigToService = async () => {
  setCommonConfig(commonConfig.value);
};

const init = async () => {
  await getCommonConfigFromService();
};

onMounted(async () => {
  await init();
});

const change = async () => {
  await setCommonConfigToService();
};

const shortCutChange = async (e) => {
  const { func, value } = e;
  shortCuts.value.forEach((item) => {
    if (item.func === func) {
      item.keys = value;
    }
  });
  let saveValue = shortCuts.value.map((item) => {
    return `${item.func}:${item.keys.join("+")}`;
  });
  commonConfig.value.hotkeys = saveValue;
  change();
};
</script>

<style scoped>
.common-config-container {
  height: 100vh;
}
</style>
