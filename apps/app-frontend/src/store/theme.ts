import { defineStore } from 'pinia'

import { DEFAULT_ACCENT_COLOR, getAccentThemeVariables, normalizeHexColor } from '@/helpers/color'

export const DEFAULT_FEATURE_FLAGS = {
	project_background: false,
	page_path: false,
	worlds_tab: false,
	worlds_in_home: true,
	servers_in_app: false,
	server_project_qa: false,
	i18n_debug: false,
}

export const THEME_OPTIONS = ['dark', 'light', 'oled', 'system'] as const

export type FeatureFlag = keyof typeof DEFAULT_FEATURE_FLAGS
export type FeatureFlags = Record<FeatureFlag, boolean>
export type ColorTheme = (typeof THEME_OPTIONS)[number]

export type ThemeStore = {
	selectedTheme: ColorTheme
	accentColor: string
	advancedRendering: boolean
	toggleSidebar: boolean

	devMode: boolean
	featureFlags: FeatureFlags
}

export const DEFAULT_THEME_STORE: ThemeStore = {
	selectedTheme: 'dark',
	accentColor: DEFAULT_ACCENT_COLOR,
	advancedRendering: true,
	toggleSidebar: false,

	devMode: false,
	featureFlags: DEFAULT_FEATURE_FLAGS,
}

export const useTheming = defineStore('themeStore', {
	state: () => DEFAULT_THEME_STORE,
	actions: {
		setThemeState(newTheme: ColorTheme) {
			if (THEME_OPTIONS.includes(newTheme)) {
				this.selectedTheme = newTheme
			} else {
				console.warn('Selected theme is not present. Check themeOptions.')
			}

			this.setThemeClass()
		},
		setAccentColor(newAccentColor: string) {
			this.accentColor = normalizeHexColor(newAccentColor)
			this.applyAccentVariables()
		},
		applyAccentVariables() {
			const resolvedTheme =
				this.selectedTheme === 'system'
					? window.matchMedia('(prefers-color-scheme: dark)').matches
						? 'dark'
						: 'light'
					: this.selectedTheme

			const themeVariables = getAccentThemeVariables(
				this.accentColor,
				resolvedTheme === 'dark' || resolvedTheme === 'oled',
			)
			const root = document.documentElement

			for (const [name, value] of Object.entries(themeVariables)) {
				root.style.setProperty(name, value)
			}
		},
		setThemeClass() {
			const root = document.documentElement

			for (const theme of THEME_OPTIONS) {
				root.classList.remove(`${theme}-mode`)
			}

			let theme = this.selectedTheme
			if (this.selectedTheme === 'system') {
				const darkThemeMq = window.matchMedia('(prefers-color-scheme: dark)')
				if (darkThemeMq.matches) {
					theme = 'dark'
				} else {
					theme = 'light'
				}
			}

			root.classList.add(`${theme}-mode`)
			this.applyAccentVariables()
		},
		getFeatureFlag(key: FeatureFlag) {
			return this.featureFlags[key] ?? DEFAULT_FEATURE_FLAGS[key]
		},
		getThemeOptions() {
			return THEME_OPTIONS
		},
	},
})
