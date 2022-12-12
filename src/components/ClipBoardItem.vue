<template>
  <li class="DocSearch-Hit" id="docsearch-item-0" role="option" :aria-selected="select">
    <a href="javascript:void(0)"
      ><div class="DocSearch-Hit-Container">
        <div class="DocSearch-Hit-content-wrapper">
          <span class="DocSearch-Hit-title">
            <template :key="item.id" v-for="item in data.contentParse">
              <mark v-if="item.match">{{ item.content }}</mark>
              <template v-else>
                {{ item.content }}
              </template>
            </template>
          </span>
        </div>
        <div class="DocSearch-Hit-action">
          <svg class="DocSearch-Hit-Select-Icon" width="20" height="20" viewBox="0 0 20 20">
            <g
              stroke="currentColor"
              fill="none"
              fill-rule="evenodd"
              stroke-linecap="round"
              stroke-linejoin="round"
            >
              <path d="M18 3v4c0 2-2 4-4 4H2"></path>
              <path d="M8 17l-6-6 6-6"></path>
            </g>
          </svg>
          <!-- <HotKeyItem :keymap="['Cmd+1']"></HotKeyItem> -->
        </div>
      </div></a
    >
  </li>
</template>
<script setup>
import HotKeyItem from "./HotKeyItem.vue";

defineProps({
  select: Boolean,
  data: Object,
});
</script>
<style scoped>
.DocSearch-Container,
.DocSearch-Container * {
  box-sizing: border-box;
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
  justify-content: space-between;
  color: var(--docsearch-hit-color);
  display: flex;
  flex-direction: row;
  min-height: var(--docsearch-hit-height);
  max-height: var(--docsearch-hit-max-height);
  padding: 0 var(--docsearch-spacing) 0 0;
  overflow: hidden;
}
.DocSearch-Hit-action,
.DocSearch-Hit-icon {
  color: var(--docsearch-muted-color);
  stroke-width: var(--docsearch-icon-stroke-width);
}
.DocSearch-Hit-content-wrapper {
  display: flex;
  flex-direction: column;
  font-weight: 500;
  justify-content: center;
  line-height: 1.4em;
  margin: 0 8px;
  overflow-x: hidden;
  position: relative;
  text-overflow: ellipsis;
  /* white-space: nowrap; */
  width: 90%;
  max-height: var(--docsearch-hit-max-height);
}
.DocSearch-Hit-title {
  font-size: 0.9em;
  overflow: hidden;
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
</style>
