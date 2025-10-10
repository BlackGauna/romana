<script setup lang="ts">
import FileSvg from "../assets/file.svg";
import GameSvg from "../assets/game.svg";

defineProps<{
  icon: IconVariant;
  title?: string;
  sideBarCollapsed: boolean;
}>();

type IconVariant = "game" | "file";
</script>

<template>
  <div
    id="sidebar-icon-container"
    class="ml-16 flex h-48 w-32 items-center justify-center"
    :class="{
      'w-full': !sideBarCollapsed,
      'justify-start': !sideBarCollapsed,
    }"
  >
    <div class="flex h-32 w-32 items-center justify-center">
      <FileSvg v-if="icon == 'file'" class="w-24" />
      <GameSvg v-if="icon == 'game'" class="w-24" />
    </div>
    <span class="text-text-primary ml-8" v-if="!sideBarCollapsed">
      {{ title }}
    </span>
  </div>
</template>
