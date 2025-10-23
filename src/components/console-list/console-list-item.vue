<script setup lang="ts">
import { useConsoleStore } from "../../stores/consoleStore"

const props = defineProps({
  name: { type: String, required: true },
  id: { type: Number, required: true },
})

const consoleStore = useConsoleStore()
</script>

<template>
  <div
    class="border-border-container hover:bg-background-active flex h-80 w-80 appearance-none items-center justify-center rounded-[8px]"
    :class="[
      consoleStore.activeConsoleId === id
        ? 'bg-background-active'
        : 'bg-sidebar',
    ]"
    @click="consoleStore.setActiveConsoleId(id)"
  >
    <span class="max-w-64 text-center wrap-break-word">{{ name }}</span>
  </div>
</template>
