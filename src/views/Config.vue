<template>
  <div class="config-container">
    <div class="left-section">
      <LeftSectionItem
        v-for="item in sectionItemListTop"
        :key="item"
        :name="t(`config.section.${item}`)"
        :active="activeSection == item"
        :icon="item"
        @click="itemClick(item)"
      />
      <div class="left-section-item-split mx-3 my-2"></div>
      <LeftSectionItem
        v-for="item in sectionItemListBottom"
        :key="item"
        :name="t(`config.section.${item}`)"
        :active="activeSection == item"
        :icon="item"
        @click="itemClick(item)"
      />
    </div>
    <div class="right-section">
      <KeepAlive>
        <component
          :is="sectionIdAndComponentNameMap[activeSection]"
        ></component>
      </KeepAlive>
    </div>
  </div>
</template>

<script setup>
import LeftSectionItem from "@/components/child/config/LeftSectionItem.vue";
import RightSectionCommonConfig from "@/components/child/config/RightSectionCommonConfig.vue";
import RightSectionAbout from "@/components/child/config/RightSectionAbout.vue";
import { ref } from "vue";
import { useI18n } from "vue-i18n";
const { t } = useI18n({
  inheritLocale: true,
  useScope: "global",
});
const sectionIdAndComponentNameMap = {
  common: RightSectionCommonConfig,
  about: RightSectionAbout,
};
const sectionItemListTop = ["common"];
const sectionItemListBottom = ["about"];
const activeSection = ref("common");
const itemClick = (name) => {
  activeSection.value = name;
};
</script>

<style scoped>
.config-container {
  display: flex;
  flex-direction: row;
  height: 100vh;
  width: 100vw;
  border-top: 1px solid rgba(0, 0, 0, 0.1);
}
.left-section {
  flex: 0.5;
  background: #f7f7f7;
  border-right: 1.1px solid rgba(0, 0, 0, 0.1);
  padding: 20px;
}

.left-section-item-split {
  height: 1px;
  background: rgba(0, 0, 0, 0.1);
}

.right-section {
  flex: 1;
  background: #fff;
}
</style>
