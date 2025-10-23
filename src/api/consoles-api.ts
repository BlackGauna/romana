import { invoke } from "@tauri-apps/api/core"
import { Console, ConsoleWithGameRoms } from "../types/console"

export async function getConsoles() {
  const result: Console[] = await invoke("get_consoles")

  console.log(result)

  return result
}

export async function getConsoleWithGameRoms(consoleName: String) {
  const result: ConsoleWithGameRoms = await invoke("get_console_game_roms", {
    consoleName: consoleName,
  })

  console.log(result)

  return result
}
