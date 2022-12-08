<template>
  <div class="key-map-bar">
    <div class="key-item">Cmd+Nmb</div>
  </div>
</template>
<script setup></script>
<style scoped>
.key-map-bar {
  height: 40px;
  width: 100%;
  user-select: none;
  display: flex;
  align-items: center;
  position: absolute;
  left: 0;
  bottom: 0;
  padding-left: 16px;
  padding-right: 16px;
  border-top: 1px solid rgba(0, 0, 0, 0.2);
  box-shadow: 0 -2px 4px rgba(0, 0, 0, 0.2);
}
</style>
