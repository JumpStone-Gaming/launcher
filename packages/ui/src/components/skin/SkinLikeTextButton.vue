<script setup lang="ts">
import { ref } from 'vue'

withDefaults(
	defineProps<{
		selected?: boolean
		tooltip?: string
	}>(),
	{
		selected: false,
		tooltip: undefined,
	},
)

const emit = defineEmits<{ (e: 'click', event: MouseEvent): void }>()

const pressed = ref(false)
</script>

<template>
	<div
		v-tooltip="tooltip ?? undefined"
		class="group relative overflow-hidden rounded-xl border-2 transition-all duration-200"
		:class="[selected ? 'border-brand' : 'border-transparent hover:border-inverted']"
	>
		<button
			class="skin-btn-bg absolute inset-0 cursor-pointer p-0 border-none group-hover:brightness-125 transition-all duration-200"
			:class="selected ? 'selected' : ''"
			@mousedown="pressed = true"
			@mouseup="pressed = false"
			@mouseleave="pressed = false"
			@click="(e) => emit('click', e)"
		></button>

		<div
			class="relative w-full h-full flex flex-col items-center justify-center pointer-events-none z-10"
		>
			<div v-if="$slots.icon" class="mb-2">
				<slot name="icon" />
			</div>
			<span class="text-md text-center px-2 text-primary">
				<slot />
			</span>
		</div>
	</div>
</template>

<style scoped lang="scss">
.skin-btn-bg {
	background: var(--color-gradient-button-bg);
}

.skin-btn-bg.selected {
	background:
		linear-gradient(
			157.61deg,
			rgba(255, 255, 255, 0.45) -76.68%,
			rgba(239, 239, 239, 0.4) -38.61%,
			rgba(209, 213, 219, 0.38) 100.4%
		),
		var(--color-bg);
}

.skin-btn-bg.selected:hover,
.group:hover .skin-btn-bg.selected {
	filter: brightness(1.15);
}
</style>
