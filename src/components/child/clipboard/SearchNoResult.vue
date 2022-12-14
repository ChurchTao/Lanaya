<template>
  <div class="box-boder f-lg text-center my-0 mx-auto py-12 px-0 w-4/5" data-tauri-drag-region>
    <div data-tauri-drag-region>
      <svg
        class="my-0 mx-auto text-gray-400"
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
    <p class="f-xl text-black mt-4">{{ noResultText }}</p>
  </div>
</template>

<script setup>
defineProps({
  noResultText: {
    type: String,
    default: "No result",
  },
});
</script>
<style scoped></style>
