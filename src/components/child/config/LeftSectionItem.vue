<template>
  <div class="left-section-item px-3 py-2 h-8" :aria-active="active" @click="itemClick">
    <div v-if="icon === 'common'" class="left-section-icon mr-3 h-8 w-8 icon-blue-bg">
      <img src="@/assets/shezhi.svg" class="block h-5 w-5" />
    </div>
    <div v-if="icon === 'about'" class="left-section-icon mr-3 h-8 w-8">
      <img src="@/assets/about.svg" class="block h-5 w-5" />
    </div>
    <div class="left-section-item-title">{{ name }}</div>
  </div>
</template>
<script setup>
const props = defineProps({
  icon: {
    type: String,
    default: "common",
  },
  name: {
    type: String,
    default: "通用",
  },
  active: {
    type: Boolean,
    default: false,
  },
});
const emit = defineEmits(["click"]);
const itemClick = () => {
  emit("click", props.name);
};
</script>

<style scoped>
.left-section-item {
  display: flex;
  align-items: center;
  border-radius: 6px;
  box-sizing: content-box;
}
.left-section-item:hover {
  background: #e6e6e6;
}
.left-section-item[aria-active="true"] {
  background: #306ee3;
}
.left-section-item[aria-active="true"] .left-section-item-title {
  color: #fff;
}
.left-section-icon {
  background: #babdc1;
  border-radius: 6px;
  display: flex;
  align-items: center;
  justify-content: center;
}
.icon-blue-bg {
  background: #80c9fa;
}
</style>
