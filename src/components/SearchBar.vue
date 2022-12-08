<template>
  <div class="search-bar">
    <img src="../assets/sousuo.svg" class="search-icon" />
    <input type="text" class="search-input" placeholder="Search" />
  </div>
</template>
<script setup></script>
<style scoped>
.search-bar {
  display: flex;
  align-items: center;
  height: 60px;
  width: 100%;
  padding-left: 16px;
  padding-right: 16px;
}
.search-icon {
  height: 32px;
  width: 32px;
  margin-right: 10px;
}
.search-input {
  height: 60px;
  line-height: 60px;
  width: 100%;
  border: none;
  outline: none;
  font-size: 16px;
}
</style>
