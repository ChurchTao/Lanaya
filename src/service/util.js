// Special Keys
const _keyMap = {
  8: { name: "backspace", keyStr: "⌫" },
  9: { name: "tab", keyStr: "Tab" },
  12: { name: "clear", keyStr: "Clear" },
  13: { name: "enter", keyStr: "↩" },
  27: { name: "esc", keyStr: "Esc" },
  32: { name: "space", keyStr: "Space" },
  37: { name: "left", keyStr: "←" },
  38: { name: "up", keyStr: "↑" },
  39: { name: "right", keyStr: "→" },
  40: { name: "down", keyStr: "↓" },
  46: { name: "delete", keyStr: "Del" },
  45: { name: "insert", keyStr: "Ins" },
  36: { name: "home", keyStr: "↖" },
  35: { name: "end", keyStr: "↘" },
  33: { name: "pageup", keyStr: "⇞" },
  34: { name: "pagedown", keyStr: "⇟" },
  20: { name: "capslock", keyStr: "⇪" },
  96: { name: "num_0", keyStr: "0" },
  97: { name: "num_1", keyStr: "1" },
  98: { name: "num_2", keyStr: "2" },
  99: { name: "num_3", keyStr: "3" },
  100: { name: "num_4", keyStr: "4" },
  101: { name: "num_5", keyStr: "5" },
  102: { name: "num_6", keyStr: "6" },
  103: { name: "num_7", keyStr: "7" },
  104: { name: "num_8", keyStr: "8" },
  105: { name: "num_9", keyStr: "9" },
  106: { name: "num_multiply", keyStr: "*" },
  107: { name: "num_add", keyStr: "+" },
  108: { name: "num_enter", keyStr: "⏎" },
  109: { name: "num_subtract", keyStr: "-" },
  110: { name: "num_decimal", keyStr: "." },
  111: { name: "num_divide", keyStr: "/" },
  188: { name: ",", keyStr: "," },
  190: { name: ".", keyStr: "." },
  191: { name: "/", keyStr: "/" },
  192: { name: "`", keyStr: "`" },
  189: { name: "-", keyStr: "-" },
  187: { name: "=", keyStr: "=" },
  186: { name: ";", keyStr: ";" },
  222: { name: "'", keyStr: "'" },
  219: { name: "[", keyStr: "[" },
  221: { name: "]", keyStr: "]" },
  220: { name: "\\", keyStr: "\\" },
};
const _modifier = {
  16: { name: "shift", keyStr: "⇧" },
  18: { name: "alt", keyStr: "⌥" },
  17: { name: "ctrl", keyStr: "⌃" },
  91: { name: "command", keyStr: "⌘" },
};

export function getShortCutShow(keyCodeArr) {
  keyCodeArr = keyCodeArr.sort((a, b) => a - b);
  let keyStr = "";
  let modifier = "";
  let normalKey = "";
  keyCodeArr.forEach((keyCode) => {
    keyCode = parseInt(keyCode);
    if (_modifier[keyCode]) {
      modifier += _modifier[keyCode].keyStr;
    } else if (_keyMap[keyCode]) {
      keyStr = _keyMap[keyCode].keyStr;
    } else {
      normalKey = String.fromCharCode(keyCode).toUpperCase();
    }
  });
  if (modifier === "" && keyStr === "") {
    return "";
  }
  // 若只有modifier，不显示
  if (modifier !== "" && keyStr === "" && normalKey === "") {
    return "";
  }
  // 若只有keyStr，不显示
  if (modifier === "" && keyStr !== "" && normalKey !== "") {
    return "";
  }
  return modifier + keyStr + normalKey;
}

// 返回数组形式
export function getShortCutShowAnyway(keyCodeArr) {
  keyCodeArr = keyCodeArr.sort((a, b) => a - b);
  let keyStr = [];
  let modifier = [];
  let normalKey = [];
  keyCodeArr.forEach((keyCode) => {
    keyCode = parseInt(keyCode);
    if (_modifier[keyCode]) {
      modifier.push(_modifier[keyCode].keyStr);
    } else if (_keyMap[keyCode]) {
      keyStr.push(_keyMap[keyCode].keyStr);
    } else {
      normalKey.push(String.fromCharCode(keyCode).toUpperCase());
    }
  });
  return [...modifier, ...keyStr, ...normalKey];
}

/**
 * 返回如：commond+shift+c 或者 Commond+Shift+C
 * @param {*} keyCodeArr
 * @param {*} isFirstWordUpperCase
 * @returns
 */
export function getShortCutName(keyCodeArr, isFirstWordUpperCase = false) {
  keyCodeArr = keyCodeArr.sort((a, b) => a - b);
  let keyStr = "";
  let modifier = "";
  let normalKey = "";
  keyCodeArr.forEach((keyCode) => {
    if (_modifier[keyCode]) {
      modifier += capitalized(_modifier[keyCode].name, isFirstWordUpperCase) + "+";
    } else if (_keyMap[keyCode]) {
      keyStr = capitalized(_keyMap[keyCode].name, isFirstWordUpperCase);
    } else {
      normalKey = capitalized(String.fromCharCode(keyCode), isFirstWordUpperCase);
    }
  });
  if (modifier === "" && keyStr === "") {
    return "";
  }
  // 若只有modifier，不显示
  if (modifier !== "" && keyStr === "" && normalKey === "") {
    return "";
  }
  // 若只有keyStr，不显示
  if (modifier === "" && keyStr !== "" && normalKey !== "") {
    return "";
  }
  return modifier + keyStr + normalKey;
}

function capitalized(name, isFirstWordUpperCase = false) {
  name = name.toLowerCase();
  if (!isFirstWordUpperCase) {
    return name;
  }
  const capitalizedFirst = name[0].toUpperCase();
  if (name.length === 1) {
    return capitalizedFirst;
  }
  const rest = name.slice(1);
  return capitalizedFirst + rest;
}
