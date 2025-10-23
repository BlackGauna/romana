<script setup lang="ts">
import { useGameStore } from "../../stores/gameStore"
import Searchbar from "../common/searchbar.vue"
import RomTableItem from "./rom-table-item.vue"

const gamesStore = useGameStore()
</script>

<template>
  <div id="search-container" class="mx-32 flex w-full flex-col pt-32">
    <div class="flex w-full flex-1 flex-col overflow-hidden">
      <div class="sticky top-0 z-20">
        <div id="searchAndFilter" class="w-240 pb-16">
          <Searchbar />
        </div>
      </div>

      <div
        class="bg-background-primary m-0 h-full w-full flex-1 overflow-y-auto p-0"
      >
        <table class="w-full flex-1">
          <thead
            class="bg-border-container sticky top-[-1px] z-10 rounded-t-[20px] leading-48"
          >
            <tr>
              <th class="w-2/5">ROM</th>
              <th>Console</th>
              <th>Year</th>
              <th>Developer</th>
            </tr>
          </thead>
          <tbody>
            <RomTableItem v-for="game in gamesStore.games" :game="game" />
          </tbody>
        </table>
      </div>
    </div>
  </div>
</template>

<style lang="css" scoped>
th:first-child {
  border-top-left-radius: 20px;
  border: 0;
}
th:last-child {
  border-top-right-radius: 20px;
}
</style>
