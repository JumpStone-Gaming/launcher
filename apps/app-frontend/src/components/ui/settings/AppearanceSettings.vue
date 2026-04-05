<script setup lang="ts">
import {
	ButtonStyled,
	Combobox,
	ThemeSelector,
	Toggle,
	defineMessages,
	useVIntl,
} from '@modrinth/ui'
import { computed, ref, watch } from 'vue'

import { DEFAULT_ACCENT_COLOR, normalizeHexColor } from '@/helpers/color'
import { get, set } from '@/helpers/settings.ts'
import { getOS } from '@/helpers/utils'
import { useTheming } from '@/store/state'
import type { ColorTheme } from '@/store/theme.ts'

const themeStore = useTheming()
const { formatMessage } = useVIntl()

const os = ref(await getOS())
const settings = ref(await get())

const normalizedAccentColor = normalizeHexColor(settings.value.accent_color)
settings.value.accent_color = normalizedAccentColor
themeStore.setAccentColor(normalizedAccentColor)

const accentColorDraft = ref(normalizedAccentColor)
const accentColorPreview = computed(() => normalizeHexColor(accentColorDraft.value))

watch(
	settings,
	async () => {
		await set(settings.value)
	},
	{ deep: true },
)

const messages = defineMessages({
	themeTitle: {
		id: 'app.settings.appearance.theme.title',
		defaultMessage: 'Color theme',
	},
	themeDescription: {
		id: 'app.settings.appearance.theme.description',
		defaultMessage: 'Select your preferred color theme for Modrinth App.',
	},
	accentTitle: {
		id: 'app.settings.appearance.accent.title',
		defaultMessage: 'Custom accent color',
	},
	accentDescription: {
		id: 'app.settings.appearance.accent.description',
		defaultMessage: 'Pick a custom hex color for buttons, highlights, and the launcher accent.',
	},
	accentInputLabel: {
		id: 'app.settings.appearance.accent.input-label',
		defaultMessage: 'Accent color hex value',
	},
	accentReset: {
		id: 'app.settings.appearance.accent.reset',
		defaultMessage: 'Reset to default',
	},
	advancedRenderingTitle: {
		id: 'app.settings.appearance.advanced-rendering.title',
		defaultMessage: 'Advanced rendering',
	},
	advancedRenderingDescription: {
		id: 'app.settings.appearance.advanced-rendering.description',
		defaultMessage:
			'Enables advanced rendering such as blur effects that may cause performance issues without hardware-accelerated rendering.',
	},
	hideNametagTitle: {
		id: 'app.settings.appearance.hide-nametag.title',
		defaultMessage: 'Hide nametag',
	},
	hideNametagDescription: {
		id: 'app.settings.appearance.hide-nametag.description',
		defaultMessage: 'Disables the nametag above your player on the skins page.',
	},
	nativeDecorationsTitle: {
		id: 'app.settings.appearance.native-decorations.title',
		defaultMessage: 'Native decorations',
	},
	nativeDecorationsDescription: {
		id: 'app.settings.appearance.native-decorations.description',
		defaultMessage: 'Use system window frame (app restart required).',
	},
	minimizeLauncherTitle: {
		id: 'app.settings.appearance.minimize-launcher.title',
		defaultMessage: 'Minimize launcher',
	},
	minimizeLauncherDescription: {
		id: 'app.settings.appearance.minimize-launcher.description',
		defaultMessage: 'Minimize the launcher when a Minecraft process starts.',
	},
	defaultLandingPageTitle: {
		id: 'app.settings.appearance.default-page.title',
		defaultMessage: 'Default landing page',
	},
	defaultLandingPageDescription: {
		id: 'app.settings.appearance.default-page.description',
		defaultMessage: 'Change the page to which the launcher opens on.',
	},
	defaultLandingPageHome: {
		id: 'app.settings.appearance.default-page.home',
		defaultMessage: 'Home',
	},
	defaultLandingPageLibrary: {
		id: 'app.settings.appearance.default-page.library',
		defaultMessage: 'Library',
	},
	jumpBackWorldsTitle: {
		id: 'app.settings.appearance.worlds.title',
		defaultMessage: 'Jump back into worlds',
	},
	jumpBackWorldsDescription: {
		id: 'app.settings.appearance.worlds.description',
		defaultMessage: 'Includes recent worlds in the "Jump back in" section on the Home page.',
	},
	toggleSidebarTitle: {
		id: 'app.settings.appearance.toggle-sidebar.title',
		defaultMessage: 'Toggle sidebar',
	},
	toggleSidebarDescription: {
		id: 'app.settings.appearance.toggle-sidebar.description',
		defaultMessage: 'Enables the ability to toggle the sidebar.',
	},
})

const defaultLandingPageOptions = computed(() => [
	{ value: 'home', label: formatMessage(messages.defaultLandingPageHome) },
	{ value: 'library', label: formatMessage(messages.defaultLandingPageLibrary) },
])

function getDefaultLandingPageLabel(value: string) {
	return value === 'library'
		? formatMessage(messages.defaultLandingPageLibrary)
		: formatMessage(messages.defaultLandingPageHome)
}

function commitAccentColor(value: string) {
	const normalized = normalizeHexColor(value)
	accentColorDraft.value = normalized
	settings.value.accent_color = normalized
	themeStore.setAccentColor(normalized)
}

function resetAccentColor() {
	commitAccentColor(DEFAULT_ACCENT_COLOR)
}
</script>

<template>
	<h2 class="m-0 text-lg font-extrabold text-contrast">{{ formatMessage(messages.themeTitle) }}</h2>
	<p class="m-0 mt-1">{{ formatMessage(messages.themeDescription) }}</p>

	<ThemeSelector
		:update-color-theme="
			(theme: ColorTheme) => {
				themeStore.setThemeState(theme)
				settings.theme = theme
			}
		"
		:current-theme="settings.theme"
		:theme-options="themeStore.getThemeOptions()"
		system-theme-color="system"
	/>

	<div class="mt-4 flex items-center justify-between gap-4">
		<div>
			<h2 class="m-0 text-lg font-extrabold text-contrast">
				{{ formatMessage(messages.accentTitle) }}
			</h2>
			<p class="m-0 mt-1">{{ formatMessage(messages.accentDescription) }}</p>
		</div>

		<div class="flex items-center gap-3">
			<label
				class="flex h-10 w-10 shrink-0 items-center justify-center overflow-hidden rounded-xl border border-solid border-button-bg bg-surface-4 p-1 transition-[border-color,box-shadow] focus-within:outline focus-within:outline-2 focus-within:outline-brand"
			>
				<span class="visually-hidden">{{ formatMessage(messages.accentInputLabel) }}</span>
				<input
					type="color"
					class="h-full w-full cursor-pointer border-0 bg-transparent p-0"
					:value="accentColorPreview"
					:aria-label="formatMessage(messages.accentInputLabel)"
					@input="(event) => commitAccentColor((event.target as HTMLInputElement).value)"
				/>
			</label>

			<input
				v-model="accentColorDraft"
				type="text"
				class="h-10 w-28 appearance-none rounded-xl border border-solid border-button-bg bg-surface-4 px-3 font-medium text-primary shadow-none transition-[shadow,color] focus:text-contrast focus:ring-4 focus:ring-brand-shadow"
				:aria-label="formatMessage(messages.accentInputLabel)"
				:placeholder="DEFAULT_ACCENT_COLOR"
				maxlength="7"
				spellcheck="false"
				@blur="commitAccentColor(accentColorDraft)"
				@keyup.enter="commitAccentColor(accentColorDraft)"
			/>

			<ButtonStyled type="transparent">
				<button
					type="button"
					:disabled="accentColorPreview === DEFAULT_ACCENT_COLOR"
					@click="resetAccentColor"
				>
					{{ formatMessage(messages.accentReset) }}
				</button>
			</ButtonStyled>
		</div>
	</div>

	<div class="mt-4 flex items-center justify-between">
		<div>
			<h2 class="m-0 text-lg font-extrabold text-contrast">
				{{ formatMessage(messages.advancedRenderingTitle) }}
			</h2>
			<p class="m-0 mt-1">{{ formatMessage(messages.advancedRenderingDescription) }}</p>
		</div>

		<Toggle
			id="advanced-rendering"
			:model-value="themeStore.advancedRendering"
			@update:model-value="
				(e) => {
					themeStore.advancedRendering = !!e
					settings.advanced_rendering = themeStore.advancedRendering
				}
			"
		/>
	</div>

	<div class="mt-4 flex items-center justify-between">
		<div>
			<h2 class="m-0 text-lg font-extrabold text-contrast">
				{{ formatMessage(messages.hideNametagTitle) }}
			</h2>
			<p class="m-0 mt-1">{{ formatMessage(messages.hideNametagDescription) }}</p>
		</div>
		<Toggle id="hide-nametag-skins-page" v-model="settings.hide_nametag_skins_page" />
	</div>

	<div v-if="os !== 'MacOS'" class="mt-4 flex items-center justify-between gap-4">
		<div>
			<h2 class="m-0 text-lg font-extrabold text-contrast">
				{{ formatMessage(messages.nativeDecorationsTitle) }}
			</h2>
			<p class="m-0 mt-1">{{ formatMessage(messages.nativeDecorationsDescription) }}</p>
		</div>
		<Toggle id="native-decorations" v-model="settings.native_decorations" />
	</div>

	<div class="mt-4 flex items-center justify-between">
		<div>
			<h2 class="m-0 text-lg font-extrabold text-contrast">
				{{ formatMessage(messages.minimizeLauncherTitle) }}
			</h2>
			<p class="m-0 mt-1">{{ formatMessage(messages.minimizeLauncherDescription) }}</p>
		</div>
		<Toggle id="minimize-launcher" v-model="settings.hide_on_process_start" />
	</div>

	<div class="mt-4 flex items-center justify-between">
		<div>
			<h2 class="m-0 text-lg font-extrabold text-contrast">
				{{ formatMessage(messages.defaultLandingPageTitle) }}
			</h2>
			<p class="m-0 mt-1">{{ formatMessage(messages.defaultLandingPageDescription) }}</p>
		</div>
		<Combobox
			id="opening-page"
			v-model="settings.default_page"
			name="Opening page dropdown"
			class="w-40"
			:options="defaultLandingPageOptions"
			:display-value="getDefaultLandingPageLabel(settings.default_page)"
		/>
	</div>

	<div class="mt-4 flex items-center justify-between">
		<div>
			<h2 class="m-0 text-lg font-extrabold text-contrast">
				{{ formatMessage(messages.jumpBackWorldsTitle) }}
			</h2>
			<p class="m-0 mt-1">{{ formatMessage(messages.jumpBackWorldsDescription) }}</p>
		</div>
		<Toggle
			:model-value="themeStore.getFeatureFlag('worlds_in_home')"
			@update:model-value="
				() => {
					const newValue = !themeStore.getFeatureFlag('worlds_in_home')
					themeStore.featureFlags['worlds_in_home'] = newValue
					settings.feature_flags['worlds_in_home'] = newValue
				}
			"
		/>
	</div>

	<div class="mt-4 flex items-center justify-between">
		<div>
			<h2 class="m-0 text-lg font-extrabold text-contrast">
				{{ formatMessage(messages.toggleSidebarTitle) }}
			</h2>
			<p class="m-0 mt-1">{{ formatMessage(messages.toggleSidebarDescription) }}</p>
		</div>
		<Toggle
			id="toggle-sidebar"
			:model-value="settings.toggle_sidebar"
			@update:model-value="
				(e) => {
					settings.toggle_sidebar = !!e
					themeStore.toggleSidebar = settings.toggle_sidebar
				}
			"
		/>
	</div>
</template>
