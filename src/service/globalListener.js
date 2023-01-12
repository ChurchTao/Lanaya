import { listen } from "@tauri-apps/api/event";
import { getCommonConfig } from "./cmds";

let commonConfigListener = null;
let commonConfigConsumerList = [];

export async function initGlobalListener() {
  await initCommonConfigListener();
}

export const registerCommonConfigConsumer = (consumer) => {
  commonConfigConsumerList.push(consumer);
};

const initCommonConfigListener = async () => {
  if (!commonConfigListener) {
    commonConfigListener = await listen("lanaya://refresh-common-config", async (event) => {
      const commonConfig = await getCommonConfig();
      console.log("new commonConfig", commonConfig);
      commonConfigConsumerList.forEach((consumer) => {
        consumer(commonConfig);
      });
    });
  }
};
