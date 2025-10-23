<script setup lang="ts">
import { useConsoleStore } from "../../stores/consoleStore"
import ConsoleListItem from "./console-list-item.vue"

const consoleStore = useConsoleStore()
</script>

<template>
  <div
    id="consoles-bar"
    class="bg-sidebar border-border-container flex h-full w-88 flex-col items-center justify-start overflow-y-auto border-r"
  >
    <div class="">
      <div class="flex h-64 items-center justify-center">
        <span>Consoles</span>
      </div>
    </div>
    <div class="bg-border-container h-1 w-full"></div>
    <div v-for="console in consoleStore.consoles" :key="console.name">
      <ConsoleListItem :name="console.name" :id="console.id" />
      <div class="bg-border-container h-1 w-full"></div>
    </div>
  </div>
</template>

<style lang="css" scoped>
#consoles-bar {
  overflow-x: hidden;
}
</style>
