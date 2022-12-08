<template>
  <div class="DocSearch-Dropdown">
    <!-- <div class="DocSearch-StartScreen"><p class="DocSearch-Help">No recent searches</p></div> -->
    <div class="DocSearch-Dropdown-Container">
      <section class="DocSearch-Hits">
        <ul role="listbox" aria-labelledby="docsearch-label" id="docsearch-list">
          <li class="DocSearch-Hit" id="docsearch-item-0" role="option" aria-selected="false">
            <a href="/en-US/component/infinite-scroll#infinite-scroll"
              ><div class="DocSearch-Hit-Container">
                <div class="DocSearch-Hit-content-wrapper">
                  <span class="DocSearch-Hit-title"><mark>In</mark>finite Scroll #</span>
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
        </ul>
      </section>
      <section class="DocSearch-HitsFooter"></section>
    </div>
  </div>
</template>
<script setup>
import HotKeyItem from "./HotKeyItem.vue";
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
</style>
