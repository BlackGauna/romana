<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core"
import { ref } from "vue"
import RomListContainer from "./components/rom-list/rom-list-container.vue"
import Sidebar from "./components/sidebar.vue"

const greetMsg = ref("")
const name = ref("")

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  greetMsg.value = await invoke("greet", { name: name.value })
}
</script>

<template>
  <div class="flex h-full w-full justify-start">
    <Sidebar />
    <RomListContainer />
  </div>
</template>

<style src="./styles/global.css"></style>
