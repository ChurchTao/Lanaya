import { createApp } from "vue";
import "./style.css";
import App from "./App.vue";
import router from "./router/router.js";
import { createI18n, useI18n } from "vue-i18n";
import messages from "@intlify/vite-plugin-vue-i18n/messages";
import { registerCommonConfigConsumer, initGlobalListener } from "./service/globalListener";
import { getCommonConfig } from "./service/cmds";
let defaultLanguage = "en";
const initConfig = await getCommonConfig();
if (initConfig.language) {
  defaultLanguage = initConfig.language;
}
const i18n = createI18n({
  locale: defaultLanguage,
  messages,
});
const app = createApp(App);
app.use(router);
app.use(i18n);
app.mount("#app");
initGlobalListener();
registerCommonConfigConsumer((config) => {
  console.log("consumer", config);
  if (config.language) {
    // not work
    i18n.global.locale = config.language;
  }
});
