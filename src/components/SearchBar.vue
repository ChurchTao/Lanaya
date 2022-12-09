<template>
  <header class="search-bar" data-tauri-drag-region>
    <img src="../assets/sousuo.svg" class="search-icon" />
    <input type="text" class="search-input" placeholder="Search" />
  </header>
</template>
<script setup></script>
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
