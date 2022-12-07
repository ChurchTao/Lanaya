<script setup>
import { ref } from "vue";
import { invoke } from "@tauri-apps/api";

defineProps({
  msg: String,
});

const count = ref(0);
const greet = () => {
  // 调用命令
  // 在应用窗口中右键，打开开发者工具
  // 你会看到控制台上输出了 "Hello, World!"！
  invoke("greet", { name: "World" })
    // `invoke` 返回的是一个 Promise
    .then((response) => console.log(response));
};
</script>

<template>
  <h1>{{ msg }}</h1>

  <div class="card">
    <button type="button" @click="count++">count is {{ count }}</button>
    <p>
      Edit
      <code>components/HelloWorld.vue</code> to test HMR
    </p>
    <button type="button" @click="greet()">greet</button>
  </div>

  <p>
    Check out
    <a href="https://vuejs.org/guide/quick-start.html#local" target="_blank">create-vue</a>, the
    official Vue + Vite starter
  </p>
  <p>
    Install
    <a href="https://github.com/johnsoncodehk/volar" target="_blank">Volar</a>
    in your IDE for a better DX
  </p>
  <p class="read-the-docs">Click on the Vite and Vue logos to learn more</p>
</template>

<style scoped>
.read-the-docs {
  color: #888;
}
</style>
