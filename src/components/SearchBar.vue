<template>
  <header class="search-bar" data-tauri-drag-region>
    <img src="../assets/sousuo.svg" class="search-icon" />
    <input
      @input="onSearchInputChange"
      @keyup.enter="onEnter"
      type="text"
      class="search-input"
      placeholder="Search"
      autocomplete="off"
    />
  </header>
</template>
<script setup>
const emit = defineEmits(["change", "enter"]);
const onSearchInputChange = (e) => {
  emit("change", e.target.value);
};
const onEnter = () => {
  emit("enter");
};
</script>
<style scoped>
.search-bar {
  display: flex;
  align-items: center;
  height: 60px;
  padding: 0 var(--docsearch-spacing);
  border-bottom: 1px solid rgba(0, 0, 0, 0.1);
}
.search-icon {
  height: 28px;
  width: 28px;
  margin-right: 5px;
}
.search-input {
  height: 60px;
  line-height: 60px;
  width: 100%;
  border: none;
  outline: none;
  font-size: 22px;
}
</style>
