<template>
  <div class="app">
    <el-container>
      <el-header>
        <div class="toolbar">
          <div class="menu">
            <span class="menu-icon" @click="openMenu('fileMenu')" @mouseover="_hideMenus()">
              <el-icon :size="icon.size" :color="icon.color">
                <Document />
              </el-icon>文件
            </span>
            <div v-show="showMeuns.fileMenu" class="menu-dropdown">
              <div @click="newFile">新建文件</div>
              <div @click="openFile">打开文件</div>
              <div @click="openFolder">打开文件夹</div>
              <div class="menu-divider" />
              <div @click="saveFile">保存</div>
              <div @click="saveAsFile">另存为</div>
            </div>
          </div>
          <div class="menu">
            <span class="menu-icon" @click="openMenu('helpMenu')" @mouseover="_hideMenus()">
              <el-icon :size="icon.size" :color="icon.color">
                <QuestionFilled />
              </el-icon>帮助
            </span>
            <div v-show="showMeuns.helpMenu" class="menu-dropdown">
              <div @click="showAbout">关于</div>
              <div @click="openDocs">文档</div>
            </div>
          </div>
        </div>
      </el-header>
      <el-main>
        <Workspace ref="workspaceView" />
      </el-main>
      <el-footer class="panel footer">
        <div class="title">JMarkText Editor Version 1.0.0</div>
      </el-footer>
    </el-container>
  </div>
  <About ref="aboutDialog" />
  <Help ref="helpDialog" />
</template>
<script setup>
import About from "./lib/about.vue";
import Help from "./lib/help.vue";
import Workspace from "./lib/workspace.vue";
</script>
<script>

import { Util } from "./util.ts";

export default {
  setup() {
    console.log("setup");
  },
  data() {
    return {
      showMeuns: {
        fileMenu: false,
        helpMenu: false,
      },
      icon: {
        size: 20,
        color: "",
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
      console.log("hideMenus");
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
    invokeMenuClickEvent(command) {
      command();
      this._hideMenus();
    },
    newFile() {
      this.invokeMenuClickEvent(() => this.$refs.workspaceView.newFile());
    },
    openFile() {
      this.invokeMenuClickEvent(() => this.$refs.workspaceView.openFile());
    },
    openFolder() {
      this.invokeMenuClickEvent(() => this.$refs.workspaceView.openFolder());
    },
    saveFile() {
      this.invokeMenuClickEvent(() => this.$refs.workspaceView.saveFile());
    },
    saveAsFile() {
      this.invokeMenuClickEvent(() => this.$refs.workspaceView.saveAsFile());
    },
    showAbout() {
      this.invokeMenuClickEvent(() => this.$refs.aboutDialog.showDialog());
    },
    openDocs() {
      this.$refs.helpDialog.showDialog();
      this._hideMenus();
    },
  },
  mounted() {
    // 点击文档其他地方关闭菜单
    document.addEventListener("click", this.closeMenus);
    window.addEventListener('keydown', (e) => {
      if ((e.ctrlKey || e.metaKey) && e.key === 's') {
        e.preventDefault(); // 阻止浏览器默认保存网页行为
         this.saveFile();
      }
    });
  },
  beforeDestroy() {
    document.removeEventListener("click", this.closeMenus);
    window.removeEventListener('keydown');
  },
};
</script>
<style scoped>
.menu-divider {
  height: 1px;
  background-color: #dcdfe6;
  padding: 0;
  pointer-events: none;
  /* 禁止点击 */
}
</style>
