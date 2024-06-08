<!-- Tree.vue -->
<template>
  <ul>
    <Tree-Node
        v-for="(value, key) in treeData"
        :key="key"
        :node="value"
    ></Tree-Node>
  </ul>
</template>

<script setup>
import TreeNode from './TreeNode.vue';

const props = defineProps({
  treeData: {
    type: Object,
    required: true,
    default: {},
  },
});
</script>

<style scoped>
ul {
  padding : 0;
  margin  : 0;
}
</style>
