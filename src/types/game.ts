import { Rom } from "./rom"

export type Game = {
  id: number
  title: string
  console_id: number
}

export type GameWithRoms = Game & { roms: Rom[] }
