<template>
  <div
    class="data-list box-border min-h-4 max-h-96 overflow-y-auto py-0 px-4 mt-3"
    data-tauri-drag-region
  >
    <SearchNoResult v-if="noResult || data.length == 0" />
    <div class="data-list-container" v-if="!noResult">
      <section class="item-hits">
        <ul role="listbox" aria-labelledby="docsearch-label" id="docsearch-list">
          <ClipBoardItem
            v-for="(item, index) in data"
            :key="index"
            :data="item"
            :select="selectIndex == index"
            @click="clickThis(index)"
            @mouseenter="selectThis(index)"
          />
        </ul>
      </section>
      <section class="item-hits-footer p-4 mb-4 flex justify-center text-base"></section>
    </div>
  </div>
</template>
<script setup>
import HotKeyItem from "./HotKeyItem.vue";
import ClipBoardItem from "./ClipBoardItem.vue";
import SearchNoResult from "./child/clipboard/SearchNoResult.vue";
import { ref, onUpdated } from "vue";
import { appWindow, LogicalSize } from "@tauri-apps/api/window";

const emit = defineEmits(["clickItem", "changeIndex"]);
defineProps({
  noResult: Boolean,
  data: Array[Object],
  selectIndex: Number,
});
const selectThis = (index) => {
  emit("changeIndex", index);
};
const clickThis = (index) => {
  emit("clickItem", index);
};

onUpdated(async () => {
  let height = document.body.offsetHeight;
  let width = document.body.offsetWidth;
  await appWindow.setSize(new LogicalSize(width, height));
});
</script>
<style scoped>
.data-list {
  scrollbar-color: var(--docsearch-muted-color) var(--docsearch-modal-background);
  scrollbar-width: thin;
}

.data-list ul {
  list-style: none;
  margin: 0;
  padding: 0;
}

.item-hits-footer {
  color: var(--docsearch-muted-color);
}
</style>
