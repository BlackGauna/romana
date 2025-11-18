<script setup lang="ts">
import { invoke } from "@tauri-apps/api/core"
import RomListContainer from "./components/rom-list/rom-list-container.vue"
import Sidebar from "./components/sidebar.vue"

async function openFileDialog(): Promise<void> {
  return await invoke("choose_rom_dir", { consoleName: "snes" })
}
</script>

<template>
  <button @click="openFileDialog">Test open file path</button>
  <div class="flex h-screen w-full justify-start select-none">
    <nav class="sticky top-0 h-fit self-start">
      <Sidebar />
    </nav>
    <RomListContainer class="h-full flex-1" />
  </div>
</template>

<style src="./styles/global.css"></style>
