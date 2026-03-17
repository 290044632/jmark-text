<template>
  <el-dialog
    v-model="centerDialogVisible"
    title="JMarkText - Markdown Editor"
    width="500"
    align-center
  >
    <span>
        <a href="https://github.com/290044632/jmark-text.git" target="_blank">JMarkText - Markdown Editor</a>
    </span>
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
