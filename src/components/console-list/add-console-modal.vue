<script setup lang="ts">
import { ref } from "vue"
import { useConsoleStore } from "../../stores/consoleStore"
import { Console } from "../../types/console"

const consoleStore = useConsoleStore()

const selected = ref<Console | null>(null)

const dropdownIsOpen = ref(false)
const select = (console: Console) => {
  selected.value = console
  dropdownIsOpen.value = false // close dropdown
}

const props = defineProps<{
  isOpen: boolean
}>()

const emits = defineEmits<{ modalClose: [void] }>()

function closeModal() {
  console.log("close")

  emits("modalClose")
}
</script>

<template>
  <div>
    <Teleport to="body">
      <!-- transparent container for handling click outside modal -->
      <div
        v-if="props.isOpen"
        class="absolute top-0 h-full w-full bg-transparent"
        @click="closeModal"
      ></div>
      <div
        v-if="props.isOpen"
        id="modal"
        class="bg-background-primary border-border-modal fixed top-1/2 left-1/2 flex h-200 w-fit -translate-1/2 rounded-[20px] border p-16"
      >
        <div>
          <div class="min-w-max bg-amber-50">
            <!-- Dropdown button -->
            <button
              @click="dropdownIsOpen = !dropdownIsOpen"
              class="bg-button-primary w-200 rounded p-2 text-left text-white"
            >
              {{ selected ? selected.name : "Select a console" }}
            </button>

            <!-- Dropdown options -->
            <ul
              v-if="dropdownIsOpen"
              class="absolute z-10 mt-1 max-h-200 min-w-max overflow-y-auto rounded border bg-white shadow"
            >
              <li
                v-for="console in consoleStore.consoles"
                :key="console.abbreviation"
                @click="select(console)"
                class="cursor-pointer p-2 hover:bg-amber-500 hover:text-white"
              >
                {{ console.name }}
              </li>
            </ul>
          </div>
        </div>
        <button
          class="bg-button-primary border-border-modal absolute right-8 bottom-4 inline-flex items-center justify-center rounded-[8px] border p-4"
          @click="closeModal"
        >
          Close
        </button>
      </div>
    </Teleport>
  </div>
</template>

<style lang="css" scoped>
button {
  font-feature-settings: "ss01";
}
</style>
