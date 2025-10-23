import { invoke } from "@tauri-apps/api/core"
import { Game, GameWithRoms } from "../types/game"

export async function getAllGames(): Promise<Game[]> {
  return await invoke("get_games")
}

export async function getGamesForConsole(
  consoleId: number,
): Promise<GameWithRoms[]> {
  let games: GameWithRoms[] = await invoke("get_game_roms_for_console", {
    consoleId: consoleId,
  })

  console.log("got new games for console", consoleId, ":", games)
  return games
}
