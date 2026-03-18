<template>
  <!-- <Editor
    :value="config.value"
    :plugins="config.plugins"
    :editorConfig="config.editorConfig"
    :locale="config.locale"
    mode="split"
    @change="handleChange"
    :placeholder="config.placeholder"
  /> -->
  <el-container class="editor">
    <el-header>
    </el-header>
    <el-main class="main">
      <el-tabs
        v-model="editableTabsValue"
        type="card"
        editable
        @edit="handleTabsEdit"
        class="tabs"
      >
        <el-tab-pane
          v-for="item in editableTabs"
          :key="item.name"
          :label="item.title"
          :name="item.name"
        >
          <Editor
            :value="item.content"
            :plugins="config.plugins"
            :editorConfig="config.editorConfig"
            :locale="config.locale"
            mode="split"
            @change="handleChange"
            :placeholder="item.placeholder"
          />
        </el-tab-pane>
      </el-tabs>
    </el-main>
    <el-footer>
    </el-footer>
  </el-container>
</template>

<script setup>
import { ref, watch } from "vue";
import gfm from "@bytemd/plugin-gfm";
import highlight from "@bytemd/plugin-highlight-ssr";
import breaks from "@bytemd/plugin-breaks";
import gemoji from "@bytemd/plugin-gemoji";
import mermaid from "@bytemd/plugin-mermaid";
import math from "@bytemd/plugin-math-ssr";
import { Editor } from "@bytemd/vue-next";
import "bytemd/dist/index.css";
import "highlight.js/styles/default.css";
import "katex/dist/katex.css";
import zh_Hans from "bytemd/locales/zh_Hans.json";

let tabIndex = 1;
const editableTabsValue = ref("1");
const editableTabs = ref([
  {
    title: "New Tab",
    name: "1",
    content: "",
    placeholder: "New Tab placeholder",
  },
]);

const handleTabsEdit = (targetName, action) => {
  if (action === "add") {
    const newTabName = `${++tabIndex}`;
    editableTabs.value.push({
      title: "New Tab",
      name: newTabName,
      content: "",
      placeholder: "New Tab placeholder",
    });
    editableTabsValue.value = newTabName;
  } else if (action === "remove") {
    const tabs = editableTabs.value;
    let activeName = editableTabsValue.value;
    if (activeName === targetName) {
      tabs.forEach((tab, index) => {
        if (tab.name === targetName) {
          const nextTab = tabs[index + 1] || tabs[index - 1];
          if (nextTab) {
            activeName = nextTab.name;
          }
        }
      });
    }

    editableTabsValue.value = activeName;
    editableTabs.value = tabs.filter((tab) => tab.name !== targetName);
    tabIndex--;
    if (tabIndex == 0) {
        handleTabsEdit('', "add");
    }
  }
};

// 定义 props，支持 v-model
const props = defineProps({
  config: {
    type: Object,
    default: () => ({
      plugins: [gfm(), highlight(), breaks(), gemoji(), math(), mermaid()],
      editorConfig: { lineNumbers: true },
      locale: zh_Hans,
      //   placeholder:
      // "这是一个基于bytemd(https://bytemd.js.org/#usage)的Markdown编辑器,感谢bytemd的开发者！",
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

const handleChange = (v) => {
  editableTabs.value[editableTabsValue.value - 1].content = v;
};
</script>
