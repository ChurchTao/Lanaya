import { createI18n } from "vue-i18n";
import en from "./locales/en.yaml";
import zh from "./locales/zh.yaml";

export const i18n = createI18n({
  locale: "zh",
  fallbackLocale: "zh",
  messages: {
    en,
    zh,
  },
});

export const i18nt = i18n.global.t;

export function setLanguage(locale) {
  i18n.global.locale = locale;
}
