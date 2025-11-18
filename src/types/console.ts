import { Game, GameWithRoms } from "./game"

export type Console = {
  id: number
  name: string
  manufacturer: string
  abbreviation: string
  inLibrary: boolean
}

export type ConsoleWithGames = Console & { games: Game[] }
export type ConsoleWithGameRoms = Console & { games: GameWithRoms[] }
