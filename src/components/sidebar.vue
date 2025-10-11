<script setup lang="ts">
import { ref } from "vue";
import { useUiState } from "../stores/uiState";
import SidebarIcon from "./sidebar-icon.vue";
import SidebarListItem from "./sidebar-list-item.vue";

const { activeUi, setActiveUi } = useUiState();

let sideBarCollapsed = ref(false);
</script>

<template>
  <div class="bg-sidebar h-lvh" :class="[sideBarCollapsed ? 'w-64' : 'w-232']">
    <div id="sidebar-icon-row" class="flex w-full justify-end">
      <SidebarIcon
        icon="sidebar"
        class="mt-16 mr-16"
        @click="sideBarCollapsed = !sideBarCollapsed"
      />
    </div>

    <div
      id="main-sidebar"
      class="mt-32 flex w-full flex-col items-center justify-center px-8"
    >
      <SidebarListItem
        icon="game"
        title="Rom Library"
        :side-bar-collapsed="sideBarCollapsed"
        :active="activeUi == 'game'"
        @click="setActiveUi('game')"
      />
      <SidebarListItem
        icon="file"
        title="DAT File Editor"
        :side-bar-collapsed="sideBarCollapsed"
        :active="activeUi == 'file'"
        @click="setActiveUi('file')"
      />
    </div>
  </div>
</template>

<style scoped>
@import "tailwindcss";
</style>
