<template>
  <div class="app">
    <div class="toolbar">
      <div class="menu">
        <span
          class="menu-icon"
          @click="openMenu('fileMenu')"
          @mouseover="openMenu('fileMenu')"
        >
          <File :size="icon.size" :color="icon.color" />文件
        </span>
        <div v-show="showMeuns.fileMenu" class="menu-dropdown">
          <div @click="newFile">新建</div>
          <div @click="openFile">打开</div>
          <div @click="saveFile">保存</div>
          <div @click="saveAsFile">另存为</div>
        </div>
      </div>
      <div class="menu">
        <span
          class="menu-icon"
          @click="openMenu('helpMenu')"
          @mouseover="openMenu('helpMenu')"
        >
          <HelpCircle :size="icon.size" :color="icon.color" />帮助
        </span>
        <div v-show="showMeuns.helpMenu" class="menu-dropdown">
          <div @click="showAbout">关于</div>
          <div @click="openDocs">文档</div>
        </div>
      </div>
    </div>
    <div class="editor-container">
      <Workspace />
    </div>
  </div>
  <About ref="aboutDialog" />
  <Help ref="helpDialog" />
</template>
<script setup>
import { File, Menu, Home, HelpCircle } from "@boxicons/vue";
import About from "./lib/about.vue";
import Help from "./lib/help.vue";
import Workspace from "./lib/workspace.vue";
</script>
<script>
// import gfm from "@bytemd/plugin-gfm";
// import highlight from "@bytemd/plugin-highlight-ssr";
// import breaks from "@bytemd/plugin-breaks";
// import gemoji from "@bytemd/plugin-gemoji";
// import mermaid from "@bytemd/plugin-mermaid";
// import math from "@bytemd/plugin-math-ssr";
// import { Editor, Viewer } from "@bytemd/vue-next";
// import "bytemd/dist/index.css";
// import "highlight.js/styles/default.css"; // 代码高亮主题
// import "katex/dist/katex.css";
// import zh_Hans from "bytemd/locales/zh_Hans.json";
import { Util } from "./util.ts";

// const plugins = [gfm(), highlight(), breaks(), gemoji(), math(), mermaid()];
// const editorConfig = {
//   lineNumbers: true,
// };

export default {
  //components: { Editor },
  setup() {
    console.log("setup");
  },
  data() {
    return {
      showMeuns: {
        fileMenu: false,
        helpMenu: false,
      },
      value: "",
      //plugins,
      //editorConfig,
     // locale: zh_Hans,
      placeholder:
        "这是一个基于bytemd(https://bytemd.js.org/#usage)的Markdown编辑器,感谢bytemd的开发者！",
      icon: {
        size: "sm",
        color: "black",
      },
    };
  },
  methods: {
    handleChange(v) {
      this.value = v;
    },
    closeMenus(e) {
      if (
        !Util.elementHasClass(e.target.parentNode, ["menu", "menu-dropdown", "menu-icon"])
      ) {
        this._hideMenus();
      }
    },
    _hideMenus(key) {
      if (key) {
        this.showMeuns[key] = false;
        return;
      }
      for (const key in this.showMeuns) {
        this.showMeuns[key] = false;
      }
    },
    _showMenus(key) {
      this._hideMenus();
      this.showMeuns[key] = true;
    },
    openMenu(key) {
      this._showMenus(key);
    },
    newFile() {
      console.log("newFile");
    },
    openFile() {
      console.log("openFile");
    },
    saveFile() {
      console.log("saveFile");
    },
    saveAsFile() {
      console.log("saveAsFile");
    },
    showAbout() {
      this.$refs.aboutDialog.showDialog();
    },
    openDocs() {
      this.$refs.helpDialog.showDialog();
    },
  },
  mounted() {
    // 点击文档其他地方关闭菜单
    document.addEventListener("click", this.closeMenus);
  },
  beforeDestroy() {
    document.removeEventListener("click", this.closeMenus);
  },
};
</script>
