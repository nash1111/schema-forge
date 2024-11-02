<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

const greetMsg = ref("");
const name = ref("");

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  greetMsg.value = await invoke("greet", { name: name.value });
}
</script>

<template>
  <div id="app">
    <header>
      <h1>My Application</h1>

    </header>
    <main>
      <router-view />
    </main>
    <footer>
      <p>© 2023 My Application</p>
    </footer>
  </div>
</template>


<style scoped>
/* 基本的なスタイルをここに追加 */
#app {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  text-align: center;
  color: #2c3e50;
  margin-top: 60px;
}

header {
  background-color: #f6f6f6;
  padding: 10px;
}

nav a {
  margin: 0 10px;
  color: #42b983;
  text-decoration: none;
}

nav a:hover {
  text-decoration: underline;
}

footer {
  margin-top: 20px;
  font-size: 0.8em;
  color: #888;
}
</style>