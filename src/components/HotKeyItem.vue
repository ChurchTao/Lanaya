<template>
  <div v-if="show" class="key-item-box">
    <kbd v-for="item in keymap" :key="item" class="kbd kbd-sm mr-px">{{
      item
    }}</kbd>
    <div class="key-tips ml-1 text-gray-500">{{ $t(tips) }}</div>
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
}
.key-tips {
  font-size: 0.75em;
  user-select: none;
  -moz-user-select: none;
  -webkit-user-select: none;
  -ms-user-select: none;
}
</style>
