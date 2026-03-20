<template>
  <el-dialog
    v-model="centerDialogVisible"
    title="JMarkText - Markdown Editor"
    width="500"
    align-center
  >
    <ul>
      <li><a href="https://github.com/290044632/jmark-text.git" target="_blank">JMarkText - Markdown Editor</a></li>
      <li><a href="https://github.com/290044632/bytemd.git" target="_blank">ByteMD - Markdown Editor</a></li>
      <li><a href="https://github.com/pd4d10/bytemd.git" target="_blank">ByteMD v1 repository</a></li>
      <li><a href="https://mermaid.js.org" target="_blank">Mermaid</a></li>
    </ul>
  </el-dialog>
</template>
<script setup>
import { ref } from "vue";
const centerDialogVisible = ref(false);
const showDialog = () => {
  centerDialogVisible.value = true;
};
defineExpose({ showDialog });
</script>
