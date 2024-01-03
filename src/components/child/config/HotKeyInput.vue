<template>
  <div
    class="hotkey-input-container w-full flex items-center justify-between py-1"
  >
    <div class="hotkey-func text-sm">{{ $t("hotkeys." + func) }}：</div>
    <div
      class="relative input-container flex items-center border rounded-lg border-gray-200 py-1 pl-2 pr-6 text-left sm:text-sm"
      :class="focus ? 'ring-2 ring-blue-500 ring-opacity-50' : ''"
    >
      <input
        :id="id"
        type="text"
        class="hotkey-input"
        :placeholder="$t('config.common.hotkeys_placeholder')"
        :value="hotkeyShow"
        autocomplete="off"
        @blur="onBlur"
        @focus="onFocus"
        @keydown="(e) => e.preventDefault()"
        @keypress="(e) => e.preventDefault()"
      />
      <div
        class="hotkey-input-clear cursor-pointer absolute inset-y-0 right-0 pl-2 pr-2 flex items-center hover:bg-gray-100"
        @click="onClear"
      >
        <img src="@/assets/delete.svg" class="h-4 w-4" />
      </div>
    </div>
  </div>
</template>

<script setup>
import hotkeys from "hotkeys-js";
import { computed, ref } from "vue";
import { getShortCutShow } from "@/service/shortCutUtil";
const id = Math.random().toString(36);
const emit = defineEmits(["change"]);
const focus = ref(false);
const onBlur = () => {
  focus.value = false;
  hotkeys.unbind("*", "input");
};
const props = defineProps({
  keys: {
    type: Array,
    // eslint-disable-next-line vue/require-valid-default-prop
    default: [],
  },
  func: {
    type: String,
    default: "",
  },
});

const hotkeyShow = computed(() => {
  if (props.keys.length > 0) {
    return getShortCutShow(props.keys);
  }
  return "";
});

const onFocus = () => {
  focus.value = true;
  hotkeys.filter = function (event) {
    var tagName = (event.target || event.srcElement).tagName;
    hotkeys.setScope(
      /^(INPUT|TEXTAREA|SELECT)$/.test(tagName) ? "input" : "other",
    );
    return true;
  };
  hotkeys(
    "*",
    {
      capture: true,
      scope: "input",
      element: document.getElementById(id),
    },
    function () {
      let keyCodes = hotkeys.getPressedKeyCodes();
      let keyShowValue = getShortCutShow(keyCodes);
      if (keyShowValue.length > 0) {
        emit("change", {
          value: keyCodes,
          func: props.func,
        });
      }
    },
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
