import {appWindow} from "@tauri-apps/api/window";
import {createApp} from "vue";
import App from "./App.vue";
import {createPinia} from "pinia";

const pinia = createPinia();
const app = createApp(App);
app.use(pinia);
app.mount("#app");
appWindow.show().then(() => {
    console.log("Window is shown")
});
