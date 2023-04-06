<template>
  <div v-if="(tags && tags.length > 0) || editable" class="text-sm mt-3">
    <div class="flex">
      <div
        v-for="(tag, i) in tags"
        :key="tag"
        class="badge text-xs mr-1"
        @click.stop="removeTag(i)"
      >
        <svg
          xmlns="http://www.w3.org/2000/svg"
          viewBox="0 0 20 20"
          fill="currentColor"
          class="w-4 h-4 mr-px"
        >
          <path
            fill-rule="evenodd"
            d="M5.5 3A2.5 2.5 0 003 5.5v2.879a2.5 2.5 0 00.732 1.767l6.5 6.5a2.5 2.5 0 003.536 0l2.878-2.878a2.5 2.5 0 000-3.536l-6.5-6.5A2.5 2.5 0 008.38 3H5.5zM6 7a1 1 0 100-2 1 1 0 000 2z"
            clip-rule="evenodd"
          />
        </svg>
        {{ tag }}
        <svg
          v-if="editable"
          xmlns="http://www.w3.org/2000/svg"
          fill="none"
          viewBox="0 0 24 24"
          class="inline-block w-4 h-4 stroke-current"
        >
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            stroke-width="2"
            d="M6 18L18 6M6 6l12 12"
          ></path>
        </svg>
      </div>

      <div v-if="editable" class="form-control ml-1">
        <div class="input-group input-group-xs" @click.prevent.stop>
          <input
            v-model="inputText"
            type="text"
            :placeholder="$t('tags.placeholder')"
            data-disable-hotkeys="true"
            class="input input-xs input-primary input-bordered w-20"
            autofocus
            autocomplete="off"
            autoCorrect="off"
            autoCapitalize="off"
            spellCheck="false"
            @keyup.enter.stop="addTags"
            @keyup.esc.stop="$emit('onEscape')"
          />
          <button class="btn btn-xs btn-square" @click="addTags">
            <svg
              xmlns="http://www.w3.org/2000/svg"
              xmlns:xlink="http://www.w3.org/1999/xlink"
              class="h-4 w-4"
              viewBox="0 0 20 20"
            >
              <g fill="none">
                <path
                  d="M10.5 2.75a.75.75 0 0 0-1.5 0V9H2.75a.75.75 0 0 0 0 1.5H9v6.25a.75.75 0 0 0 1.5 0V10.5h6.25a.75.75 0 0 0 0-1.5H10.5V2.75z"
                  fill="currentColor"
                ></path>
              </g>
            </svg>
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref } from "vue";
import { saveTags } from "../service/cmds";

const emits = defineEmits(["onEscape"]);
const props = defineProps({
  recordId: Number,
  tags: Array,
  editable: Boolean,
});

const inputText = ref("");

const removeTag = async (tag) => {
  if (!props.editable) {
    return;
  }
  const removedTags = props.tags.splice(tag, 1);
  let res = await saveTags(props.recordId, props.tags);
  if (!res) {
    props.tags.splice(tag, 0, ...removedTags);
  }
};

const addTags = async () => {
  const oldLength = props.tags.length;
  const newTags = inputText.value
    .split(",")
    .map((tag) => tag.toLowerCase().trim());
  newTags.forEach((tag) => {
    if (!!tag && !props.tags.includes(tag)) {
      props.tags.push(tag);
    }
  });
  let res = await saveTags(props.recordId, props.tags);
  if (res) {
    inputText.value = "";
  } else {
    props.tags.splice(oldLength);
  }
};
</script>

<style scoped>
.input {
  outline: none;
}
</style>
