<script setup lang="ts">
import { computed, ref, useTemplateRef, watch } from "vue"
import { type GameWithRoms } from "../../types/game"

const props = defineProps<{
  game: GameWithRoms
  openPopup?: boolean
}>()
defineEmits(["click"])

const hovered = ref(false)

const titleRef = useTemplateRef("titleRef")
const titleOverflows = computed(() => {
  if (!titleRef.value) return false

  if (titleRef.value.scrollWidth - titleRef.value.offsetWidth > 0) {
    return true
  } else {
    return false
  }
})

watch(titleRef, (value) => {
  if (!value) return
})
</script>

<template>
  <div
    class="contents"
    @mouseover="hovered = true"
    @mouseleave="hovered = false"
  >
    <div
      id="icon-and-info-container"
      class="item flex h-48 w-full items-center justify-start px-8"
      @click="$emit('click')"
    >
      <div id="icon" class="mx-8 h-32 w-32 shrink-0 border-red-100">
        <!-- TODO: load artwork, if downloaded - else placeholder -->
        <img
          class="h-full w-full bg-gradient-to-b from-orange-400 to-red-300"
        />
      </div>
      <div
        id="info"
        class="flex w-full min-w-0 flex-col justify-around"
        :class="{ 'text-text-selected': hovered }"
      >
        <div id="title" class="relative mb-4 h-14 w-full truncate leading-18">
          <div
            :class="{
              'animate-marquee': titleOverflows && hovered,
              truncate: titleOverflows && !hovered,
            }"
            ref="titleRef"
          >
            {{ props.game.title }}
          </div>
        </div>
        <span class="text-[12px]">
          {{ game.roms.length }} rom{{ game.roms.length > 1 ? "s" : "" }}
        </span>
      </div>
    </div>
    <!-- TODO: replace with metadata, when implemented -->
    <div class="item flex items-center justify-center">SNES</div>
    <div class="item flex items-center justify-center">1990</div>
    <div class="item flex items-center justify-center">Nintendo</div>
  </div>

  <div
    id="rom-popup"
    class="col-span-4 flex-col p-8"
    :class="openPopup ? 'flex' : 'hidden'"
  >
    <div
      v-for="rom in game.roms"
      class="flex h-14 items-center text-[14px] leading-[14px]"
    >
      <span class="relative top-1">{{ rom.title }}</span>
    </div>
  </div>
</template>

<style lang="css" scoped>
body {
  ascent-override: 0;
}

@keyframes marquee {
  0% {
    transform: translateX(0%);
  }
  10% {
    transform: translateX(0%);
  }
  100% {
    transform: translateX(-120%);
  }
}
@keyframes marquee2 {
  0% {
    transform: translateX(120%);
  }

  100% {
    transform: translateX(0%);
  }
}

.animate-marquee {
  animation:
    marquee 5s linear forwards,
    marquee2 5s linear 5s forwards;
}
</style>
