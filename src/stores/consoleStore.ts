import { defineStore } from "pinia"
import { computed, ref, watchEffect } from "vue"
import { getConsoles } from "../api/consoles-api"
import { Console } from "../types/console"

export const useConsoleStore = defineStore("console", () => {
  const consoles = ref<Console[]>([])
  const activeConsoleId = ref<number | null>(null)
  const isInitialized = ref(false)
  const isLoading = ref(false)

  const consolesInLibrary = computed(() =>
    consoles.value.filter((c) => c.inLibrary),
  )

  async function fetchConsoles() {
    // TODO: error handling
    if (isLoading.value) return
    isLoading.value = true
    console.log("fetching consoles")

    consoles.value = await getConsoles()

    isInitialized.value = true
    isLoading.value = false
  }

  function setActiveConsoleId(consoleId: number) {
    activeConsoleId.value = consoleId
  }

  watchEffect(async () => {
    if (!isInitialized.value) fetchConsoles()
  })

  return {
    consoles,
    fetchConsoles,
    isInitialized,
    setActiveConsoleId,
    activeConsoleId,
    consolesInLibrary,
  }
})
