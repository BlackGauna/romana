import { ref } from "vue"

const activeUi = ref<UiType>("game")
const activeConsole = ref<String | null>(null)

export function useUiState() {
  function setActiveUi(ui: UiType) {
    activeUi.value = ui
  }

  return { activeUi, setActiveUi }
}

export function useUiConsoleState() {
  function setActiveUiConsole(consoleName: String) {
    console.log("active console set to: ", consoleName)

    activeConsole.value = consoleName
  }

  return { activeConsole, setActiveUiConsole }
}

export type UiType = "game" | "file"
