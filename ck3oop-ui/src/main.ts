import { createApp } from "vue";
import App from "./App.vue";
import {appWindow} from "@tauri-apps/api/window";
import {helloWorld} from "ck3oop-core-js";

helloWorld("ck3oop-ui")
createApp(App).mount("#app");
appWindow.show().then(() => {});
