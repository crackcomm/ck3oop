import { createApp } from "vue";
import App from "./App.vue";
import {appWindow} from "@tauri-apps/api/window";

createApp(App).mount("#app");
appWindow.show().then(() => {});