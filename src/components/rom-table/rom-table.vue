<script setup lang="ts">
import Searchbar from "../common/searchbar.vue"
import RomTableItem from "./rom-table-item.vue"
</script>

<template>
  <div class="flex w-full flex-col">
    <div id="searchAndFilter" class="mb-16 w-240">
      <Searchbar />
    </div>

    <table class="w-full">
      <thead class="bg-border-container h-48 rounded-t-[20px]">
        <tr>
          <th>ROM</th>
          <th>Console</th>
          <th>Year</th>
          <th>Developer</th>
        </tr>
      </thead>
      <tbody class="w-full">
        <RomTableItem class="w-full" />
        <RomTableItem class="w-full" />
        <RomTableItem class="w-full" />
        <RomTableItem class="w-full" />
      </tbody>
    </table>
  </div>
</template>

<style lang="css" scoped>
th:first-child {
  border-top-left-radius: 20px;
  border: 0;
}
th:last-child {
  border-top-right-radius: 20px;
}
</style>
