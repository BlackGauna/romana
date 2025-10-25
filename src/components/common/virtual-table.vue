<script setup lang="ts">
import { computed, onBeforeUnmount, onMounted, ref, watch } from "vue"
import VirtualTableHeaderItem from "./virtual-table-header-item.vue"

// TODO: add props to make this a generic table component

interface Props {
  headers: string[]
  gridTemplateColumns?: Array<string | null>
  items: Object[]
}
const { headers, gridTemplateColumns = [], items } = defineProps<Props>()
// if not all column widths are supplied, fill with defaults
const gridColumns = ref(
  gridTemplateColumns
    .concat(
      Array(Math.max(headers.length - gridTemplateColumns.length, 0)).fill(
        null,
      ),
    )
    .map((value) => value ?? "minmax(auto, 1fr)")
    .join(" "),
)

const emit = defineEmits(["visibleItems"])

const rowHeight = 48
const buffer = 5

// resize observer for making the visibleCount reactive to height changes of the viewport
let resizeObserver: ResizeObserver | null = null
const viewport = ref<HTMLElement | null>(null)
const viewportHeight = ref(0)
const scrollTop = ref(0)
const visibleCount = computed(() => Math.ceil(viewportHeight.value / rowHeight))

const startIndex = computed(() =>
  Math.max(Math.floor(scrollTop.value / rowHeight) - buffer, 0),
)
const endIndex = computed(() =>
  Math.min(
    startIndex.value + visibleCount.value + 2 * buffer, // 2 * buffer because -buffer is applied to startIndex
    items.length,
  ),
)
const visibleItems = computed(() =>
  items.slice(startIndex.value, endIndex.value),
)

// offsets for fillers for enabling scrolling behavior
const topOffset = computed(() => startIndex.value * rowHeight)
const bottomOffset = computed(() =>
  Math.max((items.length - endIndex.value) * rowHeight, 0),
)

onMounted(() => {
  if (!viewport.value) return
  resizeObserver = new ResizeObserver(() => {
    viewportHeight.value = viewport.value!.clientHeight
  })
  resizeObserver.observe(viewport.value)
})

onBeforeUnmount(() => {
  resizeObserver?.disconnect()
})

watch(visibleItems, (value) => {
  emit("visibleItems", value)
})

function onScroll(e: Event) {
  scrollTop.value = (e.target as HTMLElement).scrollTop
}
</script>

<template>
  <div id="table" class="grid h-full overflow-hidden">
    <div
      id="header"
      class="grid h-64"
      :style="{
        gridColumn: 'span ' + headers.length + '/ span ' + headers.length,
      }"
    >
      <VirtualTableHeaderItem
        class="bg-border-container first:rounded-tl-[20px] last:rounded-tr-[20px]"
        v-for="header in headers"
        :name="header"
      />
    </div>

    <div
      id="virtual-scrolling-body"
      ref="viewport"
      class="bg-border-container overflow-auto"
      :style="{
        gridColumn: 'span ' + headers.length + '/ span ' + headers.length,
      }"
      @scroll="onScroll"
    >
      <div id="top-filler" :style="{ height: topOffset + 'px' }"></div>

      <div
        class="grid"
        :style="{
          gridColumn: 'span ' + headers.length + '/ span ' + headers.length,
        }"
      >
        <slot name="items"></slot>
      </div>

      <div id="bottom-filler" :style="{ height: bottomOffset + 'px' }"></div>
    </div>
  </div>
</template>

<style lang="css">
.grid {
  grid-template-columns: v-bind("gridColumns");
  gap: 1px;
}

#table {
  grid-template-rows: 64px auto;
}

.item {
  background-color: var(--color-table-background);
}
</style>
