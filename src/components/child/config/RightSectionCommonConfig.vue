<template>
  <div class="common-config-container px-4 py-4 relative">
    <div class="check-config-item h-10 mb-2 flex items-center justify-between">
      <div class="check-config-item-name text-sm">
        {{ t("config.common.enable_auto_launch") }}
      </div>
      <div class="check-config-item-value flex items-center">
        <BaseSwitch v-model="commonConfig.enable_auto_launch" @change="change" />
      </div>
    </div>
    <div class="select-config-item h-10 mb-2 flex items-center justify-between">
      <div class="select-config-item-name text-sm">
        {{ t("config.common.language") }}
      </div>
      <div class="select-config-item-value flex items-center">
        <BaseSelect
          v-model="languageSelectOption"
          :options="languageOptions"
          @change="changeLanguage"
        />
      </div>
    </div>
    <div class="select-config-item h-10 mb-2 flex items-center justify-between">
      <div class="select-config-item-name text-sm">
        {{ t("config.common.theme_mode") }}
      </div>
      <div class="select-config-item-value flex items-center">
        <BaseSelect v-model="themeSelectOption" :options="themeOptions" @change="changeTheme" />
      </div>
    </div>
    <div class="select-config-item h-10 mb-2 flex items-center justify-between">
      <div class="select-config-item-name text-sm">
        {{ t("config.common.record_limit") }}
      </div>
      <div class="select-config-item-value flex items-center">
        <BaseSelect
          v-model="recordLimitSelectOption"
          :options="recordLimitOptions"
          @change="changeRecordLimit"
        />
      </div>
    </div>
    <div class="select-config-item mt-4">
      <div class="select-config-item-name font-medium text-base mb-1">
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
import BaseSelect from "./base/BaseSelect.vue";
import BaseSwitch from "./base/BaseSwitch.vue";
import { getCommonConfig, setCommonConfig, setLanguage } from "@/service/cmds";
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
const languageOptions = ref([
  {
    name: "简体中文",
    value: "zh",
  },
  {
    name: "English",
    value: "en",
  },
]);
const languageSelectOption = ref({
  name: "简体中文",
  value: "zh",
});
const themeOptions = ref([
  {
    name: "Light",
    value: "light",
  },
  {
    name: "Dark",
    value: "dark",
  },
]);
const themeSelectOption = ref({
  name: "Light",
  value: "light",
});
const recordLimitOptions = ref([
  { name: "50", value: 50 },
  { name: "100", value: 100 },
  { name: "200", value: 200 },
  { name: "300", value: 300 },
]);
const recordLimitSelectOption = ref({
  name: "300",
  value: 300,
});

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
  if (res.language) {
    languageSelectOption.value = languageOptions.value.find((item) => {
      return item.value === res.language;
    });
  }
  if (res.theme_mode) {
    themeSelectOption.value = themeOptions.value.find((item) => {
      return item.value === res.theme_mode;
    });
  }
  if (res.record_limit) {
    recordLimitSelectOption.value = recordLimitOptions.value.find((item) => {
      return item.value == res.record_limit;
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

const changeLanguage = async (e) => {
  commonConfig.value.language = e.value;
  setLanguage(e.value);
};
const changeTheme = async (e) => {
  commonConfig.value.theme_mode = e.value;
  change();
};
const changeRecordLimit = async (e) => {
  commonConfig.value.record_limit = e.value;
  change();
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
