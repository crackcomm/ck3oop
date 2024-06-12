import {appWindow} from "@tauri-apps/api/window";
import {createApp} from "vue";
import App from "./App.vue";
import {createPinia} from "pinia";
import {useWorkspaceListStore} from "./store/workspaceList.js";
import {useAppStore} from "./store/app.js";

const pinia = createPinia();
const app = createApp(App);
app.use(pinia);
app.mount("#app");
appWindow.show().then(() => {
    console.log("Window is shown")
});

const appStore = useAppStore();
const wsListStore = useWorkspaceListStore();
// add a workspace
wsListStore.workspaces.push(
    {
        "id": "1",
        "name": "Workspace 1",
        "gameDir": "C:\\Program Files (x86)\\Steam\\steamapps\\common\\Crusader Kings III",
        "gameDataDir": " C:\\Users\\buk\\Documents\\Paradox Interactive\\Crusader Kings III",
        "allMods": [],
        "sortedMods": [],
    }
);