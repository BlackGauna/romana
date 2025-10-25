<script setup lang="ts">
import SortSvg from "../../assets/sort.svg"

const props = defineProps<{
  name: string
}>()
</script>

<template>
  <div class="flex h-full w-full items-center justify-center px-8">
    <div class="spacer flex-1"></div>
    <span class="h-fit flex-2 text-center">{{ props.name }}</span>
    <div class="flex-1">
      <SortSvg class="ml-auto h-24 min-w-24" />
    </div>
  </div>
</template>
