<template>
  <el-container class="editor">
    <el-header> </el-header>
    <el-main class="main">
      <el-tabs v-model="editableTabsValue" type="card" editable @edit="handleTabsEdit" class="tabs">
        <el-tab-pane v-for="item in editableTabs" :key="item.name" :label="item.title" :name="item.name">
          <Editor :value="item.content" :plugins="config.plugins" :editorConfig="config.editorConfig"
            :locale="config.locale" mode="split" @change="handleChange" :placeholder="item.placeholder"
            :showToolbar="config.showToolbar" :showStatus="config.showStatus" />
        </el-tab-pane>
      </el-tabs>
    </el-main>
    <el-footer> </el-footer>
  </el-container>
</template>

<script setup>
import { ref, watch } from "vue";
import gfm from "@bytemd/plugin-gfm";
import highlight from "@bytemd/plugin-highlight-ssr";
import breaks from "@bytemd/plugin-breaks";
import gemoji from "@bytemd/plugin-gemoji";
import mermaid from "@16uke.com/plugin-mermaid";
import math from "@bytemd/plugin-math-ssr";
import { Editor } from "@bytemd/vue-next";
import "bytemd/dist/index.css";
import "highlight.js/styles/default.css";
import zh_Hans from "bytemd/locales/zh_Hans.json";
import * as dialog from "@tauri-apps/plugin-dialog";
import * as fs from "@tauri-apps/plugin-fs";
import * as log from '@tauri-apps/plugin-log';
import * as path from '@tauri-apps/api/path';

let tabIndex = 1;
const editableTabsValue = ref("1");
const editableTabs = ref([
  {
    title: "New File",
    name: "1",
    content: "",
    placeholder: "欢迎使用基于ByteMD的Markdown编辑器",
    path: "",
  },
]);

const activeTab = (tabName) => {
  const tabs = editableTabs.value;
  let activeName = editableTabsValue.value;
  if (activeName === tabName) {
    return;
  }
  for (let i = 0; i < tabs.length; i++) {
    if (tabs[i].name === tabName) {
      editableTabsValue.value = tabName;
      break;
    }
  }

};

const handleTabsEdit = (targetName, action) => {
  if (action === "add") {
    const newTabName = `${++tabIndex}`;
    editableTabs.value.push({
      title: "New File",
      name: newTabName,
      content: "",
      placeholder: "欢迎使用基于ByteMD的Markdown编辑器",
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
      handleTabsEdit("", "add");
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
      showToolbar: false,
      showStatus: false,
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

const newFile = () => {
  handleTabsEdit("", "add");
};

const openFile = async (fpath) => {
  try {
    const filePath = fpath || await dialog.open({
      multiple: false,
      directory: false,
      filters: [
        {
          name: "Markdown",
          extensions: ["md"],
        },
      ],
    });
    if (!filePath) return;
    if ((await path.extname(filePath)) != "md") {
      return;
    }
    const title = await path.basename(filePath);
    const tab = editableTabs.value.filter((tab) => tab.title == title);
    if (tab && tab.length > 0){
      editableTabsValue.value = tab[0].name;
      return;
    }
    const file = await fs.open(filePath, {
      read: true,
      write: true,
    });
    const stat = await file.stat();
    const buf = new Uint8Array(stat.size);
    await file.read(buf);
    const textContents = new TextDecoder().decode(buf);
    await file.close();
    handleTabsEdit("", "add");
    const currentTable = editableTabs.value[editableTabsValue.value - 1];
    currentTable.content = textContents;
    currentTable.path = filePath;
    currentTable.title = await path.basename(filePath);
  } catch (error) {
    log.error(error);
  }

};
const writeFile = async (tab, fpath, content) => {
  tab.path = fpath;
  tab.title = await path.basename(fpath);
  await fs.writeFile(fpath, new TextEncoder().encode(tab.content));
};
const saveFile = async (saveAs) => {
  try {
    const currentTab = editableTabs.value[editableTabsValue.value - 1];
    if (!saveAs && currentTab.path) {
      await fs.writeFile(currentTab.path, new TextEncoder().encode(currentTab.content));
    } else {
      const fpath = await dialog.save({
        defaultPath: await path.homeDir(),
        filters: [
          {
            name: "Markdown",
            extensions: ["md"],
          },
        ],
      });
      const correctSuffix = fpath.lastIndexOf('.') !== -1;
      if (correctSuffix) {
        correctSuffix = fpath.endsWith('.md');
        if (!correctSuffix) {
          correctSuffix = await path.extname(fpath) == "md";
        }
      }
      log.info(`ismd: ${correctSuffix}`);
      if (!correctSuffix) {
        const dir = await path.dirname(fpath);
        const fname = await path.basename(fpath);
        const _fpath = await path.join(dir, fname + '.md');
        await writeFile(currentTab, _fpath, currentTab.content);
      } else {
        await writeFile(currentTab, fpath, currentTab.content);
      }

    }
  } catch (error) {
    log.error(error);
  }
};
defineExpose({
  newFile,
  openFile,
  saveFile,
});
</script>
