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
const treeData = reactive({});
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
      treeData.value = data;
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
  <div id="page-layout">
    <nav>
      <button @click="readBaseGame">Read Base Game</button>
      <button @click="readStats">Read Stats</button>
    </nav>
    <aside>
      <Tree :treeData="treeData.value"/>
    </aside>
    <div id="main">
      <p>{{ msg.valueOf() }}</p>
    </div>
  </div>
</template>

<style>

#page-layout {
  display               : grid;
  grid-template-columns : 250px auto;
  grid-template-rows    : auto 1fr;
  min-height            : 100vh;
  grid-template-areas   :
    "nav nav"
    "aside main";
}

nav {
  grid-area : nav;
}

aside {
  grid-area : aside;
}

#main {
  grid-area : main;
}

nav {
  padding         : 10px 20px;
  border-bottom   : 1px solid var(--color-border);
  display         : flex;
  justify-content : flex-end;
  gap             : 1rem;
}

aside {
  padding      : 20px 10px;
  border-right : 1px solid var(--color-border);
}

#main {
  padding    : 20px;
  background : var(--color-bg-dark-200);
}

</style>
