<script setup lang="ts">
import ConsoleListItem from "./console-list-item.vue";
</script>

<template>
  <div
    class="bg-sidebar border-border-container flex h-full w-88 flex-col items-center justify-start border-r"
  >
    <div class="flex h-64 items-center justify-center">
      <span>Cons</span>
    </div>
    <div class="bg-border-container h-1 w-full"></div>
    <template v-for="i in 5" :key="i">
      <ConsoleListItem />
      <div class="bg-border-container h-1 w-full"></div>
    </template>
  </div>
</template>
