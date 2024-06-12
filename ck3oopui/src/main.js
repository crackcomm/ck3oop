import {appWindow} from "@tauri-apps/api/window";
import {createApp} from "vue";
import App from "./App.vue";
import {createPinia} from "pinia";
import {useWorkspaceListStore} from "./store/workspaceList.js";
import {useAppStore} from "./store/app.js";
import { Store } from "tauri-plugin-store-api";


const pinia = createPinia();
const app = createApp(App);
app.use(pinia);
app.mount("#app");
appWindow.show().then(() => {
    console.log("Window is shown")
});

const appStore = useAppStore();
const wsListStore = useWorkspaceListStore();

wsListStore.loadWorkspaces().then(() => {
    console.log("Workspaces loaded");
});
