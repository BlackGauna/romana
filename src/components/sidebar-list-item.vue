<script setup lang="ts">
import FileSvg from "../assets/file.svg";
import GameSvg from "../assets/game.svg";
import { UiType } from "../stores/uiState";

defineProps<{
  icon: UiType;
  title?: string;
  sideBarCollapsed: boolean;
  active: boolean;
}>();
</script>

<template>
  <div
    id="sidebar-icon-container"
    class="flex h-48 w-48 items-center rounded-lg pl-8"
    :class="{
      'w-full': !sideBarCollapsed,
      'justify-start': !sideBarCollapsed,
      'bg-background-active': active,
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
