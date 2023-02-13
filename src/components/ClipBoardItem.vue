<template>
  <li
    class="data-item rounded flex pb-1 relative"
    :class="{ 'data-select': select }"
    role="option"
    :aria-selected="select"
  >
    <div class="data-item-outer cursor-pointer rounded shadow-md block pl-4 py-4 w-full">
      <div
        class="data-item-container flex flex-row items-center justify-between pr-4 overflow-hidden"
        :class="maxHeight"
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
          <span class="data-item-title overflow-hidden text-sm" v-html="dataShow"> </span>
        </div>
        <div
          class="data-item-action w-5 h-5 flex items-center rounded-full transition-all"
          :class="favClass"
        >
          <button
            @click.stop="markFav"
            class="data-item-action-button appearance-none"
            type="submit"
          >
            <svg width="20" height="20" viewBox="0 0 20 20">
              <path
                d="M10 14.2L5 17l1-5.6-4-4 5.5-.7 2.5-5 2.5 5 5.6.8-4 4 .9 5.5z"
                stroke="currentColor"
                fill-rule="evenodd"
                stroke-linejoin="round"
              ></path>
            </svg>
          </button>
        </div>
      </div>
    </div>
  </li>
</template>
<script setup>
import { computed } from "vue";
import { markFavorite } from "../service/cmds";

const props = defineProps({
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

const dataShow = computed(() => {
  if (props.data.type == "text") {
    let content = props.data.content_highlight || props.data.content;
    // content 中过转义掉html的标签 只保留 <b>和</b>不转义
    content = content.replace(/<[^b\/][^>]*>/g, "");
    return content;
  } else if (props.data.type == "image") {
    let imgObj = JSON.parse(props.data.content);
    return `<img src="data:image/png;base64,${imgObj.base64}" class="max-h-60 object-contain" />`;
  }
});

const maxHeight = computed(() => {
  if (props.data.type == "text") {
    return "max-h-28";
  } else if (props.data.type == "image") {
    return "max-h-60";
  }
});

const favClass = computed(() => {
  if (props.data.is_favorite) {
    return "fill-current text-yellow-400";
  } else {
    return "fill-none text-gray-300 hover:fill-current hover:ring-2 hover:bg-gray-200 hover:ring-gray-200 hover:bg-opacity-25 hover:ring-opacity-25";
  }
});

const markFav = async () => {
  let res = await markFavorite(props.data.id);
  if (res) {
    props.data.is_favorite = !props.data.is_favorite;
  }
};
</script>
<style scoped>
.data-item-outer {
  background: var(--docsearch-hit-background);
}
.data-item-container {
  color: var(--docsearch-hit-color);
}
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
</style>
<style>
b {
  background: none;
  color: var(--docsearch-highlight-color);
}
.data-item-action {
  stroke-width: var(--docsearch-icon-stroke-width);
}
.data-item[aria-selected="true"] .data-item-outer {
  background-color: var(--docsearch-highlight-color);
}
.data-item[aria-selected="true"] .data-item-action,
.data-item[aria-selected="true"] .data-item-icon,
.data-item[aria-selected="true"] b {
  color: var(--docsearch-hit-active-color) !important;
}
</style>
