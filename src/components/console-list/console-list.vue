<script setup lang="ts">
import { ref, watch } from "vue"
import AddCircleSvg from "../../assets/add_circle.svg"
import { useConsoleStore } from "../../stores/consoleStore"
import AddConsoleModal from "./add-console-modal.vue"
import ConsoleListItem from "./console-list-item.vue"

const consoleStore = useConsoleStore()
const modalOpen = ref(false)

watch(modalOpen, (newX) => {
  console.log("modalOpen", newX)
})
</script>

<template>
  <div
    id="consoles-bar"
    class="bg-sidebar border-border-container flex h-full w-88 flex-col items-center justify-start border-r"
  >
    <div class="bg-sidebar sticky top-0 w-full">
      <div class="flex h-64 w-full flex-col items-center justify-center">
        <span>Consoles</span>
        <button @click="modalOpen = true">
          <AddCircleSvg class="h-24 w-24" />
          <AddConsoleModal
            :is-open="modalOpen"
            @modal-close="modalOpen = false"
          />
        </button>
      </div>
    </div>
    <div class="bg-border-container h-1 w-full overflow-y-auto"></div>
    <div v-for="console in consoleStore.consolesInLibrary" :key="console.name">
      <ConsoleListItem :name="console.name" :id="console.id" />
      <div class="bg-border-container h-1 w-full"></div>
    </div>
  </div>
</template>

<style lang="css" scoped></style>
