import { ref } from "vue";

const activeUi = ref<UiType>("game");

export function useUiState() {
  function setActiveUi(ui: UiType) {
    activeUi.value = ui;
  }

  return { activeUi, setActiveUi };
}

export type UiType = "game" | "file";
