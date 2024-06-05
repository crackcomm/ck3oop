<!-- Tree.vue -->
<template>
  <ul>
    Tree!
    <Tree-Node
        v-for="node in treeData"
        :key="node.path"
        :node="node"
    >tree object</Tree-Node>
  </ul>
</template>

<script setup>
import TreeNode from './TreeNode.vue';

const props = defineProps({
  treeData: {
    type: Array,
    required: true,
    default: [
      {
        path: 'root',
        children: [
          {
            path: 'child1',
            children: [
              {
                path: 'child1.1',
                children: [],
              },
              {
                path: 'child1.2',
                children: [],
              },
            ],
          },
          {
            path: 'child2',
            children: [],
          },
        ],
      },
    ],
  },
});


</script>

<style scoped>
ul {
  padding-left: 20px;
  margin: 0;
}
</style>
