<template>
  <Listbox
    :model-value="modelValue"
    :by="compareDepartments"
    @update:modelValue="change"
  >
    <div class="relative mt-1 w-auto">
      <ListboxButton
        class="relative w-full cursor-default rounded-lg bg-gray-100 border border-gray-200 py-1 pl-2 pr-6 text-left sm:text-sm"
      >
        <span class="block truncate">{{ modelValue.name }}</span>
        <span
          class="pointer-events-none absolute inset-y-0 right-0 flex items-center pr-1"
        >
          <ChevronUpDownIcon class="h-5 w-5 text-gray-400" aria-hidden="true" />
        </span>
      </ListboxButton>

      <transition
        leave-active-class="transition duration-100 ease-in"
        leave-from-class="opacity-100"
        leave-to-class="opacity-0"
      >
        <ListboxOptions
          class="absolute z-10 mt-1 max-h-60 w-auto overflow-auto rounded-md bg-white py-1 text-base shadow-lg ring-1 ring-black ring-opacity-5 focus:outline-none sm:text-sm"
        >
          <ListboxOption
            v-for="department in props.options"
            v-slot="{ active, selected }"
            :key="department.name"
            :value="department"
            as="template"
          >
            <li
              :class="[
                active ? 'bg-blue-100 text-blue-900' : 'text-gray-900',
                'relative cursor-default select-none py-1 pl-6 pr-2',
              ]"
            >
              <span
                :class="[
                  selected ? 'font-medium' : 'font-normal',
                  'block truncate',
                ]"
                >{{ department.name }}</span
              >
              <span
                v-if="selected"
                class="absolute inset-y-0 left-0 flex items-center pl-1 text-blue-600"
              >
                <CheckIcon class="h-4 w-4" aria-hidden="true" />
              </span>
            </li>
          </ListboxOption>
        </ListboxOptions>
      </transition>
    </div>
  </Listbox>
</template>

<script setup>
import {
  Listbox,
  ListboxLabel,
  ListboxButton,
  ListboxOptions,
  ListboxOption,
} from "@headlessui/vue";
import { CheckIcon, ChevronUpDownIcon } from "@heroicons/vue/20/solid";

const props = defineProps({
  modelValue: Object,
  /**
   * @type {Array<{name: string, value: any}>}
   */
  options: {
    type: Array,
    default: () => [],
  },
});
const emit = defineEmits(["update:modelValue", "change"]);
const change = (value) => {
  emit("update:modelValue", value);
  emit("change", value);
};
function compareDepartments(a, b) {
  return a.value == b.value;
}
</script>
