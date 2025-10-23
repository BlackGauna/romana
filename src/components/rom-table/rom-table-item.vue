<script setup lang="ts">
import { ref } from "vue"
import { type GameWithRoms } from "../../types/game"

const props = defineProps<{
  game: GameWithRoms
}>()

const hovered = ref(false)
</script>

<template>
  <tr
    class="hover:bg-table-hover w-[1000px]"
    @mouseover="hovered = true"
    @mouseleave="hovered = false"
  >
    <td class="mx-8 flex h-48 items-center justify-start">
      <div id="icon" class="mx-8 h-32 w-32 border-2">
        <!-- TODO: load artwork, if downloaded - else placeholder -->
        <img
          class="h-full w-full bg-gradient-to-b from-orange-400 to-red-300"
        />
      </div>
      <div id="info" class="flex flex-col justify-center">
        <span
          id="title"
          class="mb-4"
          :class="{ 'text-text-selected': hovered }"
        >
          {{ props.game.title }}
        </span>
        <span class="h-fit text-[10px]">SNES • 1990</span>
      </div>
    </td>
    <td><div class="flex justify-center">SNES</div></td>
    <td><div class="flex justify-center">1990</div></td>
    <td><div class="flex justify-center">Nintendo</div></td>
  </tr>
</template>

<style lang="css" scoped></style>
