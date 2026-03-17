<template>
    <el-splitter>
        <el-splitter-panel size="30%" collapsible="true">
          <el-tree
            :data="folderView"
            :props="defaultProps"
            @node-click=""
          />
        </el-splitter-panel>
        <el-splitter-panel :min="200">
          <Editor />
        </el-splitter-panel>
    </el-splitter>
</template>
<script setup>
import { ref } from "vue";
import Editor from "./editor.vue";
</script>
