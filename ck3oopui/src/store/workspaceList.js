import {defineStore} from "pinia";
import {watch} from "vue";

const useWorkspaceListStore = defineStore('workspaceCollection', {
    state: () => ({
        workspaces: [],
    }),
    actions: {},
});

export {useWorkspaceListStore};