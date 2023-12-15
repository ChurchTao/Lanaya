import { createI18n } from "vue-i18n";
import en from "./locales/en.yaml";
import zh from "./locales/zh.yaml";

let language = window.localStorage.getItem("language") || "zh";

export const i18n = createI18n({
  locale: language,
  fallbackLocale: language,
  legacy: false,
  messages: {
    en,
    zh,
  },
});

export const i18nt = i18n.global.t;

export function setLanguage(locale) {
  i18n.global.locale.value = locale;
  window.localStorage.setItem("language", locale);
}
