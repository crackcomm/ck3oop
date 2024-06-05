<script setup>
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import Greet from "./components/Greet.vue";
import {dialog} from "@tauri-apps/api";
import {fs} from "@tauri-apps/api";
import {path} from "@tauri-apps/api";
import {reactive, ref, triggerRef} from "vue";
import Tree from './components/Tree.vue';
import _ from 'lodash';
import {metadata} from "tauri-plugin-fs-extra-api";
import {invoke} from "@tauri-apps/api/tauri";

const hasBeenRead = ref(false);
const msg = ref("Hello, Vite!");
const treeData = reactive([]);
const allEntries = ref([]);

function collectAllEntries(data) {
  const result = [];

  function recurse(entries) {
    for (const entry of entries) {
      result.push(entry);
      if (entry.children && entry.children.length > 0) {
        recurse(entry.children);
      }
    }
  }

  recurse(data);
  return result;
}

async function readBaseGame() {
  dialog.open({
    directory: true,
  }).then(async (result) => {
    console.log(result);
    // iterate over the dir
    const start = performance.now()
    invoke("recursive_walk", { path: result}).then((data)=>{
      const end = performance.now()
      msg.value = `Reading took ${end - start} ms for ${data.length} entries`;
      console.log(msg.value);
      console.log(data)
    })
    // const entries = await fs.readDir(result, {
    //   recursive: true,
    // }).then((entries) => {
    //   const end = performance.now()
    //   allEntries.value = collectAllEntries(entries);
    //   msg.value = `Reading took ${end - start} ms for ${allEntries.value.length} entries`;
    //   console.log(msg.value);
    //   treeData.value = entries;
    //   const start2 = performance.now()
    //   const end2 = performance.now()
    //   console.log(`Triggering took ${end2 - start2} ms`)
    //   console.log(treeData.value)
    //
    // })
    // read the file
  }).catch((error) => {
    console.error(error);
  });
}

async function readStats() {
  const start3 = performance.now()

  await Promise.all(allEntries.value.map(async (entry, index) => {
    await metadata(entry.path);
    console.log(`Metadata for ${index}`);
  }))

  const end3 = performance.now()
  console.log(`Metadata took ${end3 - start3} ms`)
  msg.value = `Metadata took ${end3 - start3} ms`;
}

</script>

<template>

  <div class="container">
    <h1>Welcome to Tauri!</h1>
    <p>Click on the Tauri, Vite, and Vue logos to learn more.</p>
    <button @click="readBaseGame">Read Base Game</button>
    <button @click="readStats">Read Stats</button>
    <p>{{ msg.valueOf() }}</p>
    <Tree :treeData="treeData.value"/>
    <Greet/>
  </div>
</template>

<style scoped>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.vue:hover {
  filter: drop-shadow(0 0 2em #249b73);
}

:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.container {
  margin: 0;
  padding-top: 10vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
}

.logo {
  height: 6em;
  padding: 1.5em;
  will-change: filter;
  transition: 0.75s;
}

.logo.tauri:hover {
  filter: drop-shadow(0 0 2em #24c8db);
}

.row {
  display: flex;
  justify-content: center;
}

a {
  font-weight: 500;
  color: #646cff;
  text-decoration: inherit;
}

a:hover {
  color: #535bf2;
}

h1 {
  text-align: center;
}

input,
button {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  color: #0f0f0f;
  background-color: #ffffff;
  transition: border-color 0.25s;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
}

button {
  cursor: pointer;
}

button:hover {
  border-color: #396cd8;
}

button:active {
  border-color: #396cd8;
  background-color: #e8e8e8;
}

input,
button {
  outline: none;
}

#greet-input {
  margin-right: 5px;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }

  a:hover {
    color: #24c8db;
  }

  input,
  button {
    color: #ffffff;
    background-color: #0f0f0f98;
  }

  button:active {
    background-color: #0f0f0f69;
  }
}

</style>
