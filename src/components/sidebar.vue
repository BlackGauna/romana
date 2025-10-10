<script setup lang="ts">
import { ref } from "vue";
import SidebarIcon from "./sidebar-icon.vue";
import SidebarListItem from "./sidebar-list-item.vue";

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

    <div id="main-sidebar" class="mt-32 flex w-full flex-col justify-center">
      <SidebarListItem
        icon="game"
        title="Rom Library"
        :side-bar-collapsed="sideBarCollapsed"
      />
      <SidebarListItem
        icon="file"
        title="DAT File Editor"
        :side-bar-collapsed="sideBarCollapsed"
      />
    </div>
  </div>
</template>

<style scoped>
@import "tailwindcss";
</style>
