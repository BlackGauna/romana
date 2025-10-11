<script setup lang="ts">
import ConsoleList from "../console-list/console-list.vue"
import RomTable from "../rom-table/rom-table.vue"
</script>

<template>
  <div
    id="main-container"
    class="bg-background-primary flex h-dvh w-dvw items-start"
  >
    <ConsoleList />
    <RomTable class="mx-32 mt-32" />
  </div>
</template>
