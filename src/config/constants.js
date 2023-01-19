export const languageOptions = [
  {
    name: "简体中文",
    value: "zh",
  },
  {
    name: "English",
    value: "en",
  },
];

export const themeOptions = [
  {
    name: "Light",
    value: "light",
  },
  {
    name: "Dark",
    value: "dark",
  },
];

export const recordLimitOptions = [
  { name: "50", value: 50 },
  { name: "100", value: 100 },
  { name: "200", value: 200 },
  { name: "300", value: 300 },
];

/**
 *  { keymap: ["⏎"], tips: "hotkeys.copy" },
    { keymap: ["⌘"], tips: "hotkeys.quick-copy" },
    { keymap: ["↑", "↓"], tips: "hotkeys.move-selected" },
    { keymap: ["Esc"], tips: "hotkeys.close-window" },
 */
export const defaultHotkeys = [
  {
    func: "copy",
    keys: [13],
  },
  {
    func: "quick-copy",
    keys: [91],
  },
  {
    func: "move-selected",
    keys: [38, 40],
  },
  {
    func: "close-window",
    keys: [27],
  },
];
