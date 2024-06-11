<script setup>
import {Navigation} from "./navigation.js";

const navigation = new Navigation();

</script>

<template>
  <div id="nav">
    <button
        v-for="view in navigation.views"
        :key="view.url"
        @click="navigation.setCurrentView(view.url)"
    >
      {{ view.view }}
    </button>
    <div>
      <KeepAlive>
        <component :is="navigation.currentComponent.value"/>
      </KeepAlive>
    </div>
  </div>
  <div id="page-layout">
    <nav>
    </nav>
    <div id="main">
    </div>
  </div>
</template>

<style>

#page-layout {
  display: grid;
  grid-template-columns: 250px auto;
  grid-template-rows: auto 1fr;
  min-height: 100vh;
  grid-template-areas:
    "nav nav"
    "aside main";
}

nav {
  grid-area: nav;
}

aside {
  grid-area: aside;
}

#main {
  grid-area: main;
}

nav {
  padding: 10px 20px;
  border-bottom: 1px solid var(--color-border);
  display: flex;
  justify-content: flex-end;
  gap: 1rem;
}

aside {
  padding: 20px 10px;
  border-right: 1px solid var(--color-border);
}

#main {
  padding: 20px;
  background: var(--color-bg-dark-200);
}

</style>
