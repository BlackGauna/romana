<script setup lang="ts">
import { onKeyPressed, useFocus } from "@vueuse/core"
import { ref, useTemplateRef } from "vue"
import SearchIcon from "../../assets/search.svg"

const emit = defineEmits(["search"])
let searchText = ref("")
const input = useTemplateRef<HTMLInputElement>("input")
const { focused } = useFocus(input)

let debounceTimeout: number | null = null
function emitSearch() {
  if (debounceTimeout != null) {
    clearTimeout(debounceTimeout)
  }
  debounceTimeout = setTimeout(() => {
    console.log(searchText.value)
    emit("search", searchText.value)
  }, 500)
}
</script>

<template>
  <div
    class="flex h-42 w-full rounded-[14px] bg-white inset-shadow-gray-400"
    :class="[
      focused
        ? [
            'outline-4',
            'outline-background-active',
            'inset-shadow-[2px_2px_8px]',
          ]
        : '',
    ]"
    @click="focused = true"
  >
    <div class="ml-8 flex items-center justify-center">
      <SearchIcon class="h-24 w-24" />
    </div>
    <input
      class="text-text-secondary ml-4 w-full appearance-none focus:outline-none"
      ref="input"
      v-model="searchText"
      placeholder="Search..."
      @keypress="onKeyPressed('Enter', emitSearch)"
      @input="emitSearch"
    />
  </div>
</template>
