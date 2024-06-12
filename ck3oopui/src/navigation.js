// src/navigation.js
import { ref, computed } from 'vue';
import Home from './views/Home.vue';
import Editor from "./views/Editor.vue";
import WorkspaceList from "./views/WorkspaceList.vue";

export class Navigation {
    constructor() {
        this.views = [
            { url: '/', view: 'Home', component: Home },
            { url: '/editor', view: 'Editor', component: Editor},
            { url: '/workspaceList', view: 'WorkspaceList', component: WorkspaceList },
        ];
        this.currentUrl = ref('/');
        this.currentView = ref('Home');
        this.componentInstances = {};
    }

    setCurrentView(url) {
        this.currentUrl.value = url;
        const view = this.views.find(v => v.url === url);
        this.currentView.value = view ? view.view : 'Home';
    }

    get currentComponent() {
        return computed(() => {
            const view = this.views.find(v => v.view === this.currentView.value);
            return view ? view.component : Home;
        });
    }
}