<!-- TreeNode.vue -->
<template>
  <li :class="{ 'has-children': hasChildren, 'is-opened': isOpen }">
    <p @click="toggle" class="name">
      <img v-if="hasChildren" src="/src/assets/arrow.svg" class="arrow-icon" width="14" height="14">
      <img v-if="node.is_dir" src="/src/assets/folder.svg" class="type-icon" width="16" height="16">
      <span>{{ node.name }}</span>
    </p>
    <ul v-if="isOpen">
      <tree-node
          v-for="(value, key) in sortChildren(node.children)"
          :key="key"
          :node="value"
      ></tree-node>
    </ul>
  </li>
</template>

<script setup>
import {computed, ref} from 'vue';
import _ from 'lodash';

function sortChildren(children) {
  let ordered = {}
  let entries = Object.entries(children);

  entries.sort((a, b) => a[0].localeCompare(b[0]))

  entries.sort((a, b) => {
    const aIsDir = a[1].is_dir ? 1 : 0;
    const bIsDir = b[1].is_dir ? 1 : 0;

    if (aIsDir !== bIsDir) {
      return bIsDir - aIsDir;
    }

    return 0;
  });

  for (let [key, value] of entries) {
    ordered[key] = value;
  }

  return ordered
}

const props = defineProps({
  node: {
    type: Object,
    required: true,
  },
});

const isOpen = ref(false);

const toggle = () => {
  isOpen.value = !isOpen.value;
};

const hasChildren = computed(() => {
  return Object.keys(props.node.children).length > 0;
});
</script>

<style scoped>
ul {
  padding-left : 10px;
  margin       : 5px 0;
}

li {
  list-style   : none;
  margin       : 5px 0;
  color        : inherit;
  padding-left : 0;
}

.name {
  margin         : 0;
  padding-bottom : 0;
  cursor         : pointer;
  user-select    : none;
  display        : flex;
  align-items    : center;
  gap            : 5px;
  position       : relative;
  padding-left   : 15px;
  border-radius  : var(--radius);
}

.name:hover {
  background : var(--color-bg-light-200);
}

.name span {
  display       : block;
  overflow      : hidden;
  white-space   : nowrap;
  text-overflow : ellipsis;
}

.name .arrow-icon {
  position : absolute;
  left     : 0;
}

li.is-opened > .name .arrow-icon {
  transform : rotate(90deg);
}

</style>
