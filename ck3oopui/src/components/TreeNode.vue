<!-- TreeNode.vue -->
<template>
  <li>
    <span @click="toggle">{{ node.name }}</span>
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
import {ref} from 'vue';
import _ from 'lodash';

function sortChildren(children) {
  let ordered = {}
  let entries = Object.entries(children);


  entries.sort((a, b) => a[0].localeCompare(b[0]))

  entries.sort((a, b) => {
    const aIsDir = a[1].is_dir? 1 : 0;
    const bIsDir = b[1].is_dir? 1 : 0;

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

</script>

<style scoped>
li {
  list-style: none;
  margin: 5px 0;
}

span {
  cursor: pointer;
  user-select: none;
}

ul {
  padding-left: 20px;
  margin: 5px 0;
}
</style>
