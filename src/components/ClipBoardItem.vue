<template>
  <li class="data-item rounded flex pb-1 relative" role="option" :aria-selected="select">
    <div class="data-item-outer cursor-pointer rounded shadow-md block pl-4 py-4 w-full">
      <div
        class="data-item-container flex flex-row items-center justify-between max-h-28 pr-4 overflow-hidden"
      >
        <div
          v-if="cmdPressDown && idx < 9"
          class="data-item-icon-copy w-5 h-5 text-center leading-5 rounded"
        >
          <span class="text-sm font-medium">{{ idx + 1 }}</span>
        </div>
        <div v-else class="data-item-icon w-5 h-5">
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
        <div class="data-item-content-wrapper font-medium relative mx-2">
          <span class="data-item-title overflow-hidden text-sm">
            <template :key="item.id" v-for="item in data.contentParse">
              <mark v-if="item.match">{{ item.content }}</mark>
              <template v-else>
                {{ item.content }}
              </template>
            </template>
          </span>
        </div>
        <div class="data-item-action flex items-center w-5 h-5">
          <svg
            class="data-item-select-icon w-4 h-4 hidden"
            width="20"
            height="20"
            viewBox="0 0 20 20"
          >
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
        </div>
      </div>
    </div>
  </li>
</template>
<script setup>
import HotKeyItem from "./HotKeyItem.vue";

defineProps({
  select: {
    type: Boolean,
    default: false,
  },
  data: Object,
  cmdPressDown: {
    type: Boolean,
    default: false,
  },
  idx: Number,
});
</script>
<style scoped>
.data-item-outer {
  background: var(--docsearch-hit-background);
}
.data-item-container {
  color: var(--docsearch-hit-color);
}
.data-item-action,
.data-item-icon {
  color: var(--docsearch-muted-color);
  stroke-width: var(--docsearch-icon-stroke-width);
}
.data-item-icon-copy {
  color: white;
  background: var(--docsearch-muted-color);
}
.data-item-content-wrapper {
  width: 90%;
  display: -webkit-box;
  -webkit-box-pack: center;
  word-break: break-all;
  -webkit-box-orient: vertical;
  -webkit-line-clamp: 5;
  overflow: hidden;
  text-overflow: ellipsis;
}

.text-sm {
  line-height: unset;
}

mark {
  background: none;
  color: var(--docsearch-highlight-color);
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
