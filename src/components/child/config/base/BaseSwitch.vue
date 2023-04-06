<template>
  <Switch
    :model-value="modelValue"
    :class="modelValue ? 'bg-blue-600' : 'bg-blue-200'"
    class="relative inline-flex h-[28px] w-[52px] shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out focus:outline-none focus-visible:ring-2 focus-visible:ring-white focus-visible:ring-opacity-75"
    @update:modelValue="change"
  >
    <span
      aria-hidden="true"
      :class="modelValue ? 'translate-x-6' : 'translate-x-0'"
      class="pointer-events-none inline-block h-[24px] w-[24px] transform rounded-full bg-white shadow-lg ring-0 transition duration-200 ease-in-out"
    />
  </Switch>
</template>

<script setup>
import { Switch } from "@headlessui/vue";

const props = defineProps({
  modelValue: Boolean,
});

const emit = defineEmits(["update:modelValue", "change"]);
const change = (value) => {
  emit("update:modelValue", value);
  emit("change", value);
};
</script>
