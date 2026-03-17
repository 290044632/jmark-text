<template>
  <Editor
    :value="config.value"
    :plugins="config.plugins"
    :editorConfig="config.editorConfig"
    :locale="config.locale"
    mode="split"
    @change="handleChange"
    :placeholder="config.placeholder"
  />
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

// 定义 props，支持 v-model
const props = defineProps({
    config: {
      type: Object,
      default: () => ({
        value: "",
        plugins: [gfm(), highlight(), breaks(), gemoji(), math(), mermaid()],
        editorConfig: { lineNumbers: true },
        locale: zh_Hans,
        placeholder: "这是一个基于bytemd(https://bytemd.js.org/#usage)的Markdown编辑器,感谢bytemd的开发者！",
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
  config.value.value = v;
  emit("update:config", config.value);
};

</script>
