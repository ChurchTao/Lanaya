<template>
  <div class="about-container px-4 py-4 relative">
    <div class="about-icon text-center flex justify-center mt-10">
      <img src="@/assets/about-icon.png" class="w-28 h-28" />
    </div>
    <div class="about-text mt-2">
      <p class="text-center text-base font-normal">{{ appName }}</p>
      <p class="text-center text-sm text-gray-500 font-normal">
        {{ $t("config.about.version") }} {{ appVersion }}
      </p>
    </div>
    <div
      class="about-check-update border border-gray-200 shadow-sm h-16 mt-6 rounded-lg px-4 w-full flex items-center justify-between"
    >
      <div class="change-log text-sm text-blue-700 underline cursor-pointer font-normal">
        {{ $t("config.about.change-log") }}
      </div>
      <div class="check-update">
        <button
          class="bg-white text-sm text-blue-700 border border-blue-400 px-2 py-1 rounded-lg font-normal hover:bg-blue-200"
        >
          {{ $t("config.about.check-update") }}
        </button>
      </div>
    </div>
    <div class="about-footer mt-6">
      <p class="text-sm text-blue-600 underline cursor-pointer font-normal text-center">Github</p>
      <p class="thanks mt-2 text-gray-500 text-xs text-center font-light">
        {{ $t("config.about.thanks") }}
      </p>
    </div>
  </div>
</template>

<script setup>
import { getName, getVersion } from "@tauri-apps/api/app";
import { onMounted, ref } from "vue";
const appName = ref("");
const appVersion = ref("");

onMounted(async () => {
  appName.value = await getName();
  appVersion.value = await getVersion();
});
</script>

<style scoped>
.about-container {
  height: 100vh;
}
</style>
