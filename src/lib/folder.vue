<template>
  <el-scrollbar>
    <el-tree :data="config.data" @node-click="" />
  </el-scrollbar>
</template>
<script setup>
import { ref, watch } from "vue";
// 定义 props，支持 v-model
const props = defineProps({
  config: {
    type: Object,
    default: () => ({
      data: [],
    }),
  },
});
const emit = defineEmits(["update:config"]);
const config = ref(props.config);
watch(
  () => props.config,
  (newVal) => {
    config.value = newVal;
  }
);
const openFile = () => {
    for (let i = 1; i <= 100; i++) {
      config.value.data.push({
        label: "new file" + i,
        children: [
          {
            label: "new file1阿大赛诉讼诉讼诉讼诉讼诉讼诉讼诉讼诉讼诉讼诉讼诉讼诉讼诉讼诉讼诉讼",
          },
        ],
      });
    }

};
defineExpose({
  openFile,
});
</script>
