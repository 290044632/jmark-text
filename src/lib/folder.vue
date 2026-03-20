<template>
  <el-scrollbar>
    <el-tree :data="config.data" @node-click="dblClick">
    <template #empty>
      <!-- 自定义空状态内容 -->
      <div class="custom-empty">
        <el-button  @click="openFile"> <el-icon><Document /></el-icon>打开文件</el-button>
      </div>
      <div class="custom-empty">
        <el-button  @click="openFolder"> <el-icon><Folder /></el-icon>打开文件夹</el-button>
      </div>
    </template>
    <template #icon="{ node, data }">
    <!-- 根据节点层级或类型显示不同图标 -->
    <el-icon v-if="data.isFolder">
      <Folder />
    </el-icon>
    <el-icon v-else>
      <Document />
    </el-icon>
  </template>
    </el-tree>
  </el-scrollbar>
</template>
<script setup>
import { ref, watch } from "vue";
import * as dialog from "@tauri-apps/plugin-dialog";
import * as fs from "@tauri-apps/plugin-fs";
import * as log from '@tauri-apps/plugin-log';
import * as path from '@tauri-apps/api/path';
import { lo, pa } from "element-plus/es/locale/index.mjs";
import * as tauri from '@tauri-apps/api/core';
import { Document } from "@element-plus/icons-vue";
// 定义 props，支持 v-model
const props = defineProps({
  config: {
    type: Object,
    default: () => ({
      data: [],
      editor: null,
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
const dblClick = async (data, node, e) => {
  try {
    if (!data.is_folder) {
      await config.value.editor.openFile(data.path);
    }
  } catch (error) {
    console.error(error);
    log.error(error);
  }

};
const openFile = async () => {
  await config.value?.editor.openFile();
};
const openFolder = async () => {
  try {
    const dir = await dialog.open({
      directory: true,
      multiple: false,
    });
    if (!dir) return;
    const resolvedDir = await path.resolve(dir);
    const children = await tauri.invoke('read_dir_recursive', { path: resolvedDir });
    config.value.data = [{
      label: await path.basename(resolvedDir),
      children: children
    }];
  } catch (error) {
    log.error(error);
  }

};
defineExpose({
  openFolder,
});
</script>
<style scoped>
.custom-empty {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 20px;
  color: #909399;
}
</style>