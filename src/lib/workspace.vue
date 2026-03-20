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
          <FolderView ref="folderView" :config="folderConfig"/>
        </el-main>
      </el-container>
    </el-splitter-panel>
    <el-splitter-panel :min="200">
      <EditorView ref="editorView" />
    </el-splitter-panel>
  </el-splitter>
</template>
<script setup>
import { ref, onMounted } from "vue";
import EditorView from "./editor.vue";
import FolderView from "./folder.vue";
const folderView = ref(null);
const editorView = ref(null);
const folderConfig = ref({ editor: null });
onMounted(() => {
    folderConfig.value.editor = editorView.value;
});
const openFile = () => {
    editorView.value?.openFile();
};
const newFile = () => {
    editorView.value?.newFile();
};
const openFolder = () => {
    folderView.value?.openFolder();
};
const saveFile = () => {
    editorView.value?.saveFile();
};
const saveAsFile = () => {
    editorView.value?.saveFile(true);
};
defineExpose({ openFile, newFile, openFolder, saveFile, saveAsFile });
</script>
