<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import FilesView from "./templates/files.vue";
import TaskbarView from "./templates/taskbar.vue"
import InfobarView from "./templates/infobar.vue";
import EditorView from "./templates/editor.vue";
import ViewerView from "./templates/viewer.vue"

const greetMsg = ref("");
const name = ref("");

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  greetMsg.value = await invoke("greet", { name: name.value });
}
</script>

<template>
  <main>
    <TaskbarView />
    <div class="window">
      <FilesView />
      <EditorView />
      <ViewerView />
    </div>
    <InfobarView />
  </main>
</template>

<style>
    @import url('./assets/style.css');
    @import url('./assets/files.css');
    @import url('./assets/taskbar.css');
    @import url('./assets/infobar.css');
    @import url('./assets/editor.css');
    @import url('./assets/viewer.css');
    @import url('https://fonts.googleapis.com/css2?family=Source+Code+Pro:ital,wght@0,200..900;1,200..900&display=swap');
</style>