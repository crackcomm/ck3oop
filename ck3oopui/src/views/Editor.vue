<script setup>
import {dialog} from "@tauri-apps/api";
import Tree from '../components/Tree.vue';
import _ from 'lodash';
import {invoke} from "@tauri-apps/api/tauri";

const msg = ref("Hello, Vite!");
const treeData = reactive({});


import {EditorState} from "@codemirror/state"
import {EditorView, keymap, lineNumbers} from "@codemirror/view"
import {defaultKeymap} from "@codemirror/commands"
import {onMounted, reactive, ref} from "vue";


onMounted(() => {
  const editor = new EditorView({
    parent: document.getElementById('main'),
    state: EditorState.create({
      doc: "Hello World",
      extensions: [
        keymap.of(defaultKeymap),
        lineNumbers(),
        EditorView.lineWrapping,
      ]
    }),
  });
});

async function readModList() {
  dialog.open({
    directory: true,
    recursive: true,
  }).then(async (result) => {
    console.log(result);
    const start = performance.now()
    invoke("get_mod_list", {path: result}).then((data) => {
      const end = performance.now()
      msg.value = `Reading took ${end - start} ms`;
      console.log(data)
      let treeData2 = {...treeData.value};
      _.map(data.mods, (mod, index) => {
        console.log(mod)
        if (index < 1) {
          // treeData2[mod.mod_info.name] = mod.file_tree;
        }
      })
      console.log("done")
      console.log(treeData2)
      treeData.value = treeData2;
      // tell vue to update
    })
  })
}

async function readBaseGame() {
  dialog.open({
    directory: true,
    recursive: true,
  }).then(async (result) => {
    console.log(result);
    const start = performance.now()
    invoke("recursive_walk", {path: result}).then((data) => {
      const end = performance.now()
      msg.value = `Reading took ${end - start} ms for ${data.length} entries`;
      console.log(msg.value);
      treeData.value = data;
    })
  }).catch((error) => {
    console.error(error);
  });
}

</script>

<template>
  <div id="page-layout">
    <nav>
      <button @click="readBaseGame">Read Base Game</button>
      <button @click="readModList">Read Mod List</button>
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
  display: grid;
  grid-template-columns: 250px auto;
  grid-template-rows: auto 1fr;
  min-height: 100vh;
  grid-template-areas:
    "nav nav"
    "aside main";
}

nav {
  grid-area: nav;
}

aside {
  grid-area: aside;
}

#main {
  grid-area: main;
}

nav {
  padding: 10px 20px;
  border-bottom: 1px solid var(--color-border);
  display: flex;
  justify-content: flex-end;
  gap: 1rem;
}

aside {
  padding: 20px 10px;
  border-right: 1px solid var(--color-border);
}

#main {
  padding: 20px;
  background: var(--color-bg-dark-200);
}

</style>
