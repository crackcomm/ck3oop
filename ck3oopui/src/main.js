import {createApp} from "vue";
import App from "./App.vue";
import {appWindow} from "@tauri-apps/api/window";

createApp(App).mount("#app");
appWindow.show().then(() => {

});

import {EditorState} from "@codemirror/state"
import {EditorView, gutter, keymap, lineNumbers} from "@codemirror/view"
import {defaultKeymap} from "@codemirror/commands"

let editor = new EditorView({
    parent: document.body,

    state: EditorState.create({
        doc: "Hello World",
        extensions: [
            keymap.of(defaultKeymap),
            lineNumbers(),
        ]
    }),

});
