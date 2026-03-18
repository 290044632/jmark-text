<template>
  <el-splitter>
    <el-splitter-panel size="20%" collapsible="true">
      <el-container class="explorer">
        <el-header class="panel header">
          <div class="title">
            <el-icon :size="20">
              <Folder /> 
            </el-icon>
            资源管理器
          </div>
        </el-header>
        <el-main class="panel folder">
          <FolderView ref="folderView" />
        </el-main>
        <el-footer class="panel footer">
          <div class="title">JMarkText Editor Version 1.0.0</div>
        </el-footer>
      </el-container>
    </el-splitter-panel>
    <el-splitter-panel :min="200">
      <EditorView ref="editorView" />
    </el-splitter-panel>
  </el-splitter>
</template>
<script setup>
import { ref } from "vue";
import EditorView from "./editor.vue";
import FolderView from "./folder.vue";
const folderView = ref(null);
const openFile = () => {
    folderView.value?.openFile();
};

defineExpose({ openFile });
</script>
