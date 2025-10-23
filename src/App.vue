<script setup lang="ts">
import RomListContainer from "./components/rom-list/rom-list-container.vue"
import Sidebar from "./components/sidebar.vue"
</script>

<template>
  <div class="flex h-screen w-full justify-start select-none">
    <nav class="sticky top-0 h-fit self-start">
      <Sidebar />
    </nav>
    <RomListContainer class="h-full flex-1" />
  </div>
</template>

<style src="./styles/global.css"></style>
