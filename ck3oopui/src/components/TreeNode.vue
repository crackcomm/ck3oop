<!-- TreeNode.vue -->
<template>
  <li>
    <span @click="toggle">{{ node.path }}</span>
    <ul v-if="isOpen">
      <tree-node
          v-for="child in node.children"
          :key="child.path"
          :node="child"
      ></tree-node>
    </ul>
  </li>
</template>

<script setup>
import { ref } from 'vue';

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
