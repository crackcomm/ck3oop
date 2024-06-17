<script setup>
import {useWorkspaceListStore} from "../store/workspaceList.js";
import {reactive, ref} from "vue";
import {dialog} from "@tauri-apps/api";
import {save} from "@tauri-apps/api/dialog";

// Workspace
const wsListStore = useWorkspaceListStore();
const workspace = reactive({});

// Dialogs
const selectDirectory = (defaultPath, wsProperty)=>{
  dialog.open({
    directory: true,
    recursive: true,
    defaultPath: defaultPath,
  }).then(result => {
    if (result !== null) {
      workspace.value[wsProperty] = result;
    }
  });
}

const selectGameDir = ()=>{
  selectDirectory("", 'gameDir');
}

const selectGameDataDir = ()=>{
  selectDirectory("", 'gameDataDir');
}

const saveWorkspace = async()=>{
  await wsListStore.saveWorkspaces();
}

</script>

<template>
  <div id="page-layout">
    <nav>
    </nav>
    <aside>
      <div v-for="ws in wsListStore.workspaces" :key="ws.id">
        <div @click="workspace.value=ws">{{ws.name}}</div>
      </div>
    </aside>
    <div v-if=workspace.value id="main">
      Id: {{workspace.value.id}}<br>
      Name: <input v-model="workspace.value.name"><br>
      GameDir: {{workspace.value.gameDir}}<br>
      <button @click="selectGameDir">Select Game Dir</button><br>
      GameDataDir: {{workspace.value.gameDataDir}}<br>
      <button @click="selectGameDataDir">Select Game Data Dir</button><br>
      <br>
      <button @click="saveWorkspace">Save</button>
    </div>
  </div>
</template>

<style scoped>

</style>
