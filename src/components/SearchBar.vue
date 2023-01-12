<template>
  <div class="search-bar flex items-center h-16 px-4" data-tauri-drag-region>
    <img src="../assets/sousuo.svg" class="h-8 w-8 mr-2" />
    <input
      @input="onSearchInputChange"
      @keyup.enter="onEnter"
      @focus="onFocus"
      type="text"
      class="search-input h-16 text-2xl w-full"
      :placeholder="$t('search.placeholder')"
      autocomplete="off"
    />
  </div>
</template>
<script setup>
const emit = defineEmits(["change", "enter", "focus"]);
const onSearchInputChange = (e) => {
  emit("change", e.target.value);
};
const onEnter = () => {
  emit("enter");
};
const onFocus = (e) => {
  emit("focus", e);
};
</script>
<style scoped>
.search-bar {
  background: #fafafa;
  border-bottom: 1px solid rgba(0, 0, 0, 0.1);
}
</style>
