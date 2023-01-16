<template>
  <div class="hotkey-input-container flex items-center">
    <div class="hotkey-func">{{ $t("hotkeys." + func) }}</div>
    <input
      :id="id"
      type="text"
      class="hotkey-input"
      :placeholder="$t('config.common.hotkeys_placeholder')"
      :value="hotkeyShow"
      @blur="onBlur"
      @focus="onFocus"
      autocomplete="off"
    />
    <div class="hotkey-input-clear" @click="onClear">
      <img src="@/assets/delete.svg" class="h-4 w-4" />
    </div>
  </div>
</template>

<script setup>
import hotkeys from "hotkeys-js";
import { onMounted, computed } from "vue";
import { getShortCutShow, getShortCutName } from "@/service/util";
const id = Math.random().toString(36);
const emit = defineEmits(["change"]);
const onBlur = (e) => {
  hotkeys.unbind("*", "input");
};
const props = defineProps({
  keys: {
    type: Array,
    default: [],
  },
  func: {
    type: String,
    default: "",
  },
});
onMounted(() => {
  document.getElementById(id).addEventListener("keypress", (e) => {
    e.preventDefault();
  });
});

const hotkeyShow = computed(() => {
  console.log(props.keys);
  if (props.keys.length > 0) {
    return getShortCutShow(props.keys);
  }
  return "";
});

const onFocus = (e) => {
  hotkeys.filter = function (event) {
    var tagName = (event.target || event.srcElement).tagName;
    hotkeys.setScope(/^(INPUT|TEXTAREA|SELECT)$/.test(tagName) ? "input" : "other");
    return true;
  };
  hotkeys(
    "*",
    {
      capture: true,
      scope: "input",
      element: document.getElementById(id),
    },
    function (event, handler) {
      let keyCodes = hotkeys.getPressedKeyCodes();
      let keyShowValue = getShortCutShow(keyCodes);
      if (keyShowValue.length > 0) {
        emit("change", {
          value: keyCodes,
          func: props.func,
        });
      }
    }
  );
};

const onClear = () => {
  emit("change", {
    value: [],
    func: props.func,
  });
};
</script>

<style scoped></style>
