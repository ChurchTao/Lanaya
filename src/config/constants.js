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

export const hotkeys_func_enum = {
  COPY: "copy",
  QUICK_COPY: "quick-copy",
  MOVE_SELECTED: "move-selected",
  CLOSE_WINDOW: "close-window",
  GLOBAL_SHORTCUT: "global-shortcut",
  CLEAR_HISTORY: "clear-history",
};

/**
 *  { keymap: ["⏎"], tips: "hotkeys.copy" },
    { keymap: ["⌘"], tips: "hotkeys.quick-copy" },
    { keymap: ["↑", "↓"], tips: "hotkeys.move-selected" },
    { keymap: ["Esc"], tips: "hotkeys.close-window" },
 */
export const defaultHotkeys = [
  {
    func: hotkeys_func_enum.COPY,
    keys: [13],
  },
  {
    func: hotkeys_func_enum.QUICK_COPY,
    keys: [91],
  },
  {
    func: hotkeys_func_enum.MOVE_SELECTED,
    keys: [38, 40],
  },
  {
    func: hotkeys_func_enum.CLOSE_WINDOW,
    keys: [27],
  },
  {
    func: hotkeys_func_enum.CLEAR_HISTORY,
    keys: [],
  },
  {
    func: hotkeys_func_enum.GLOBAL_SHORTCUT,
    keys: [],
  },
];
