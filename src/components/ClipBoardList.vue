<template>
  <div class="DocSearch-Dropdown" data-tauri-drag-region>
    <div class="DocSearch-StartScreen" v-if="!noResult && data.length == 0">
      <p class="DocSearch-Help">No recent record</p>
    </div>
    <div class="DocSearch-NoResults" v-if="noResult">
      <div class="DocSearch-Screen-Icon">
        <svg
          width="40"
          height="40"
          viewBox="0 0 20 20"
          fill="none"
          fill-rule="evenodd"
          stroke="currentColor"
          stroke-linecap="round"
          stroke-linejoin="round"
        >
          <path
            d="M15.5 4.8c2 3 1.7 7-1 9.7h0l4.3 4.3-4.3-4.3a7.8 7.8 0 01-9.8 1m-2.2-2.2A7.8 7.8 0 0113.2 2.4M2 18L18 2"
          ></path>
        </svg>
      </div>
      <p class="DocSearch-Title">No results</p>
    </div>
    <div class="DocSearch-Dropdown-Container" v-if="!noResult">
      <section class="DocSearch-Hits">
        <ul role="listbox" aria-labelledby="docsearch-label" id="docsearch-list">
          <ClipBoardItem
            v-for="(item, index) in data"
            :key="index"
            :data="item"
            :select="select == index"
            @mouseenter="selectThis(index)"
          />
        </ul>
      </section>
      <section class="DocSearch-HitsFooter"></section>
    </div>
  </div>
</template>
<script setup>
import HotKeyItem from "./HotKeyItem.vue";
import ClipBoardItem from "./ClipBoardItem.vue";
import { ref } from "vue";

defineProps({
  noResult: Boolean,
  data: Array[Object],
});

const select = ref(-1);
const selectThis = (index) => {
  select.value = index;
};
</script>
<style scoped>
.DocSearch-Container,
.DocSearch-Container * {
  box-sizing: border-box;
}
.DocSearch-Dropdown {
  max-height: calc(
    var(--docsearch-modal-height) - var(--docsearch-searchbox-height) - var(--docsearch-spacing) -
      var(--docsearch-footer-height)
  );
  min-height: var(--docsearch-spacing);
  overflow-y: auto;
  overflow-y: overlay;
  padding: 0 var(--docsearch-spacing);
  scrollbar-color: var(--docsearch-muted-color) var(--docsearch-modal-background);
  scrollbar-width: thin;
  margin-top: 10px;
}

.DocSearch-ErrorScreen,
.DocSearch-NoResults,
.DocSearch-StartScreen {
  font-size: 0.9em;
  margin: 0 auto;
  padding: 36px 0;
  text-align: center;
  width: 80%;
}
.DocSearch-Dropdown ul {
  list-style: none;
  margin: 0;
  padding: 0;
}
.DocSearch-Hit {
  border-radius: 4px;
  display: flex;
  padding-bottom: 4px;
  position: relative;
}
.DocSearch-Hit a {
  background: var(--docsearch-hit-background);
  border-radius: 4px;
  box-shadow: var(--docsearch-hit-shadow);
  display: block;
  padding-left: var(--docsearch-spacing);
  width: 100%;
}
.DocSearch-Hit-Container {
  align-items: center;
  color: var(--docsearch-hit-color);
  display: flex;
  flex-direction: row;
  height: var(--docsearch-hit-height);
  padding: 0 var(--docsearch-spacing) 0 0;
}
.DocSearch-Hit-action,
.DocSearch-Hit-icon {
  color: var(--docsearch-muted-color);
  stroke-width: var(--docsearch-icon-stroke-width);
}
.DocSearch-Screen-Icon {
  color: var(--docsearch-muted-color);
  padding-bottom: 12px;
}
.DocSearch-Hit-content-wrapper {
  display: flex;
  flex: 1 1 auto;
  flex-direction: column;
  font-weight: 500;
  justify-content: center;
  line-height: 1.2em;
  margin: 0 8px;
  overflow-x: hidden;
  position: relative;
  text-overflow: ellipsis;
  white-space: nowrap;
  width: 80%;
}
.DocSearch-Hit-title {
  font-size: 0.9em;
}
.DocSearch-Hits mark {
  background: none;
  color: var(--docsearch-highlight-color);
}

.DocSearch-Hit-action {
  align-items: center;
  display: flex;
  height: 22px;
  width: 22px;
}
.DocSearch-HitsFooter {
  color: var(--docsearch-muted-color);
  display: flex;
  font-size: 0.85em;
  justify-content: center;
  margin-bottom: var(--docsearch-spacing);
  padding: var(--docsearch-spacing);
}

.DocSearch-Hit[aria-selected="true"] a {
  background-color: var(--docsearch-highlight-color);
}
.DocSearch-Hit[aria-selected="true"] .DocSearch-Hit-action,
.DocSearch-Hit[aria-selected="true"] .DocSearch-Hit-icon,
.DocSearch-Hit[aria-selected="true"] .DocSearch-Hit-path,
.DocSearch-Hit[aria-selected="true"] .DocSearch-Hit-text,
.DocSearch-Hit[aria-selected="true"] .DocSearch-Hit-title,
.DocSearch-Hit[aria-selected="true"] .DocSearch-Hit-Tree,
.DocSearch-Hit[aria-selected="true"] mark {
  color: var(--docsearch-hit-active-color) !important;
}

.DocSearch-Help,
.DocSearch-Title {
  color: var(--docsearch-muted-color);
  font-size: 1.2em;
}
</style>
