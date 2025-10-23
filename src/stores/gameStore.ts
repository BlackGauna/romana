import { defineStore } from "pinia"
import { ref, watch } from "vue"
import { getGamesForConsole } from "../api/games-api"
import { GameWithRoms } from "../types/game"
import { useConsoleStore } from "./consoleStore"

export const useGameStore = defineStore("game", () => {
  const consoleStore = useConsoleStore()

  const games = ref<GameWithRoms[]>([])

  const isInitialized = ref(false)
  const isLoading = ref(false)

  async function fetchGames() {
    // TODO: error handling
    if (isLoading.value || !consoleStore.activeConsoleId) return

    isLoading.value = true
    console.log("fetching games")
    games.value = await getGamesForConsole(consoleStore.activeConsoleId)

    isInitialized.value = true
    isLoading.value = false
  }

  watch(
    () => consoleStore.activeConsoleId,
    (newId, oldId) => {
      if (!newId) return
      if (newId !== oldId) {
        console.log("new console id", newId)
        fetchGames()
      }
    },
    { immediate: true },
  )

  return { games, fetchGames, isInitialized }
})
