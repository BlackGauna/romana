<script setup lang="ts">
import { ref } from "vue"
import { useGameStore } from "../../stores/gameStore"
import { GameWithRoms } from "../../types/game"
import Searchbar from "../common/searchbar.vue"
import VirtualTable from "../common/virtual-table.vue"
import RomTableItem from "./rom-table-item.vue"

const gamesStore = useGameStore()
const visibleGames = ref<GameWithRoms[]>([])

const headers = ["ROM", "Console", "Year", "Developer"]
const openPopupGameId = ref<number | null>(null)

function handleGameClick(gameId: number) {
  openPopupGameId.value = openPopupGameId.value != gameId ? gameId : null
}
</script>

<template>
  <div class="flex h-full w-full flex-col p-24">
    <div id="searchAndFilter" class="mb-16 w-240">
      <Searchbar />
    </div>
    <div class="min-h-0 flex-1">
      <VirtualTable
        class="border-border-container rounded-[20px] border"
        :headers="headers"
        :items="gamesStore.games"
        :grid-template-columns="['minmax(200px, 2fr)']"
        @visible-items="(value) => (visibleGames = value)"
      >
        <template #items>
          <RomTableItem
            @click="handleGameClick(game.id)"
            v-for="game in visibleGames"
            :game="game"
            :open-popup="openPopupGameId == game.id"
          />
        </template>
      </VirtualTable>
    </div>
  </div>
</template>

<style lang="css" scoped></style>
