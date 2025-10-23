<script setup lang="ts">
import ConsoleList from "../console-list/console-list.vue"
import RomTable from "../rom-table/rom-table.vue"
</script>

<template>
  <div id="main-container" class="bg-background-primary flex w-dvw items-start">
    <ConsoleList
      class="sticky top-0 h-screen overflow-y-auto overscroll-contain"
    />
    <RomTable class="h-full flex-1 overflow-y-auto" />
  </div>
</template>
