import {defineStore} from "pinia";
import {reactive, ref, watch} from "vue";
import {Store} from "tauri-plugin-store-api";

const useWorkspaceListStore = defineStore('workspaceList', () => {
    const persistentStore = new Store(".workspaceList.json");
    const workspaces = ref([]);

    async function loadWorkspaces() {
        console.log("Loading workspaces");
        const loadedWorkspaces = await persistentStore.get("workspaces");
        console.log("Loaded workspaces:", loadedWorkspaces)
        if (loadedWorkspaces) {
            console.log("Setting workspaces")
            workspaces.value = loadedWorkspaces;
            console.log(workspaces.value)
        }
    }

    async function saveWorkspaces() {
        console.log("Saving workspaces");
        await persistentStore.set("workspaces", workspaces.value);
        await persistentStore.save();
    }

    return {workspaces, loadWorkspaces, saveWorkspaces};
});


export {useWorkspaceListStore};
