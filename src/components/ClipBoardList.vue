<template>
  <div
    class="data-list box-border min-h-4 overflow-y-auto px-3 pt-2"
    :data-mouseenter="mouseenter"
    @mouseenter="() => (mouseenter = true)"
    @mouseleave="() => (mouseenter = false)"
  >
    <SearchNoResult v-if="noResult || data.length == 0" />
    <div class="data-list-container" v-if="!noResult">
      <section class="item-hits">
        <ul role="listbox" aria-labelledby="docsearch-label" id="docsearch-list">
          <ClipBoardItem
            v-for="(item, index) in data"
            :key="index"
            :idx="index"
            :data="item"
            :select="selectIndex == index"
            :cmd-press-down="cmdPressDown"
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
const mouseenter = ref(false);
defineProps({
  noResult: Boolean,
  data: Array[Object],
  selectIndex: Number,
  cmdPressDown: Boolean,
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
  max-height: 30rem;
  background: #fafafa;
  -webkit-transition: all 1s;
  transition: all 1s;
}

.data-list ul {
  list-style: none;
  margin: 0;
  padding: 0;
}

.item-hits-footer {
  color: var(--docsearch-muted-color);
}
.data-list::-webkit-scrollbar {
  width: 5px;
}
.data-list::-webkit-scrollbar-corner {
  background-color: transparent;
}
.data-list::-webkit-scrollbar-thumb {
  border-radius: 5px;
  background: #9e9fa1;
  opacity: 0.9;
}
.data-list[data-mouseenter="false"]::-webkit-scrollbar-thumb {
  opacity: 0;
}
.data-list::-webkit-scrollbar-thumb:hover {
  background: #666263;
}
.data-list::-webkit-scrollbar-track {
  border-radius: 0;
}
</style>
