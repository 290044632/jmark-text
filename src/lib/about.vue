<template>
  <el-dialog
    v-model="centerDialogVisible"
    title="JMarkText - Markdown Editor"
    width="500"
    align-center
  >
    <span>Version 1.0.0</span>
  </el-dialog>
</template>
<script setup>
import { ref } from "vue";
const centerDialogVisible = ref(false);
const showDialog = () => {
  centerDialogVisible.value = true;
}
defineExpose({ showDialog });
</script>
