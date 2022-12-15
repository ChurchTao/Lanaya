<template>
  <li class="data-item rounded-sm flex pb-1 relative" role="option" :aria-selected="select">
    <div class="data-item-outer cursor-pointer rounded-sm shadow-md block pl-4 w-full">
      <div
        class="data-item-container flex flex-row items-center justify-between max-h-28 pr-4 overflow-hidden"
      >
        <div class="data-item-icon">
          <svg width="20" height="20" viewBox="0 0 20 20">
            <path
              d="M17 6v12c0 .52-.2 1-1 1H4c-.7 0-1-.33-1-1V2c0-.55.42-1 1-1h8l5 5zM14 8h-3.13c-.51 0-.87-.34-.87-.87V4"
              stroke="currentColor"
              fill="none"
              fill-rule="evenodd"
              stroke-linejoin="round"
            ></path>
          </svg>
        </div>
        <div class="data-item-content-wrapper">
          <span class="data-item-title">
            <template :key="item.id" v-for="item in data.contentParse">
              <mark v-if="item.match">{{ item.content }}</mark>
              <template v-else>
                {{ item.content }}
              </template>
            </template>
          </span>
        </div>
        <div class="data-item-action">
          <svg class="data-item-select-icon" width="20" height="20" viewBox="0 0 20 20">
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
      </div>
    </div>
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
.data-item-outer {
  background: var(--docsearch-hit-background);
}
.data-item-container {
  color: var(--docsearch-hit-color);
  min-height: var(--docsearch-hit-height);
}
.data-item-action,
.data-item-icon {
  color: var(--docsearch-muted-color);
  stroke-width: var(--docsearch-icon-stroke-width);
}

.data-item-icon {
  height: 20px;
  width: 20px;
}

.data-item-action svg {
  display: block;
  height: 18px;
  width: 18px;
}
svg.data-item-select-icon {
  display: none;
}

.data-item-content-wrapper {
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
.data-item-title {
  font-size: 0.9em;
  overflow: hidden;
}
.data-items mark {
  background: none;
  color: var(--docsearch-highlight-color);
}

.data-item-action {
  align-items: center;
  display: flex;
  height: 22px;
  width: 22px;
}

.data-item[aria-selected="true"] .data-item-outer {
  background-color: var(--docsearch-highlight-color);
}
.data-item[aria-selected="true"] .data-item-action,
.data-item[aria-selected="true"] .data-item-icon,
.data-item[aria-selected="true"] .data-item-path,
.data-item[aria-selected="true"] .data-item-text,
.data-item[aria-selected="true"] .data-item-title,
.data-item[aria-selected="true"] .data-item-Tree,
.data-item[aria-selected="true"] mark {
  color: var(--docsearch-hit-active-color) !important;
}
.data-item[aria-selected="true"] .data-item-select-icon {
  display: block;
}
</style>
