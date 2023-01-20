<template>
  <div class="key-item-box" v-if="show">
    <div class="key-item" v-for="item in keymap" :key="item">{{ item }}</div>
    <div class="key-tips">{{ $t(tips) }}</div>
  </div>
</template>
<script setup>
defineProps({
  tips: String,
  keymap: Array[String],
  show: Boolean,
});
</script>
<style scoped>
.key-item-box {
  display: flex;
  align-items: center;
  margin-right: 0.8em;
  height: 24px;
  user-select: none;
  -moz-user-select: none;
  -webkit-user-select: none;
  -ms-user-select: none;
  cursor: pointer;
}
.key-item {
  align-items: center;
  background: rgba(125, 125, 125, 0.1);
  border-radius: 2px;
  box-shadow: inset 0 -2px 0 0 #cdcde6, inset 0 0 1px 1px #fff, 0 1px 2px 1px rgba(30, 35, 90, 0.4);
  display: flex;
  height: 18px;
  justify-content: center;
  margin-right: 0.3em;
  padding: 1px 2px;
  color: #909399;
  border: 0;
  font-size: 0.75em;
  min-width: 18px;
  box-sizing: border-box;
  user-select: none;
  -moz-user-select: none;
  -webkit-user-select: none;
  -ms-user-select: none;
  cursor: pointer;
}
.key-tips {
  color: #909399;
  font-size: 0.75em;
  height: 22px;
  box-sizing: border-box;
  user-select: none;
  -moz-user-select: none;
  -webkit-user-select: none;
  -ms-user-select: none;
  cursor: pointer;
}
</style>
