import {createApp} from "vue";
import App from "./App.vue";
import {appWindow} from "@tauri-apps/api/window";

const app = createApp(App)
app.mount("#app");

appWindow.show().then(() => {

});

import {EditorState} from "@codemirror/state"
import {EditorView, gutter, keymap, lineNumbers} from "@codemirror/view"
import {defaultKeymap} from "@codemirror/commands"

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

window.editor = editor;