export const DEFAULT_ACCENT_COLOR = '#6d1bd9'

type CssVariableMap = Record<string, string>

export function normalizeHexColor(value: string | null | undefined) {
	if (!value) return DEFAULT_ACCENT_COLOR

	const trimmed = value.trim()
	if (!trimmed) return DEFAULT_ACCENT_COLOR

	const match = trimmed.match(/^#?([0-9a-f]{3}|[0-9a-f]{6})$/i)
	if (!match) return DEFAULT_ACCENT_COLOR

	const hex = match[1].toLowerCase()
	if (hex.length === 3) {
		return `#${hex
			.split('')
			.map((component) => `${component}${component}`)
			.join('')}`
	}

	return `#${hex}`
}

function hexToRgb(value: string) {
	const hex = normalizeHexColor(value).slice(1)
	return {
		r: Number.parseInt(hex.slice(0, 2), 16),
		g: Number.parseInt(hex.slice(2, 4), 16),
		b: Number.parseInt(hex.slice(4, 6), 16),
	}
}

function rgbaFromHex(value: string, alpha: number) {
	const { r, g, b } = hexToRgb(value)
	return `rgba(${r}, ${g}, ${b}, ${alpha})`
}

function mixHexColors(firstColor: string, secondColor: string, weight: number) {
	const normalizedWeight = Math.min(1, Math.max(0, weight))
	const first = hexToRgb(firstColor)
	const second = hexToRgb(secondColor)

	const mix = (firstValue: number, secondValue: number) =>
		Math.round(firstValue + (secondValue - firstValue) * normalizedWeight)

	return `#${[mix(first.r, second.r), mix(first.g, second.g), mix(first.b, second.b)]
		.map((component) => component.toString(16).padStart(2, '0'))
		.join('')}`
}

export function getAccentThemeVariables(accentColor: string, isDarkTheme: boolean): CssVariableMap {
	const normalizedAccentColor = normalizeHexColor(accentColor)
	const fadedColor = mixHexColors(
		normalizedAccentColor,
		isDarkTheme ? '#ffffff' : '#000000',
		isDarkTheme ? 0.28 : 0.16,
	)
	const strongColor = mixHexColors(
		normalizedAccentColor,
		isDarkTheme ? '#f5efff' : '#1d1730',
		isDarkTheme ? 0.18 : 0.22,
	)
	const loadingGradientEnd = mixHexColors(normalizedAccentColor, '#8f3dff', 0.5)

	return {
		'--color-brand': normalizedAccentColor,
		'--color-brand-highlight': rgbaFromHex(normalizedAccentColor, isDarkTheme ? 0.25 : 0.22),
		'--color-brand-shadow': rgbaFromHex(normalizedAccentColor, 0.7),
		'--brand-gradient-bg': `linear-gradient(0deg, ${rgbaFromHex(normalizedAccentColor, isDarkTheme ? 0.18 : 0.12)} 0%, ${rgbaFromHex(fadedColor, isDarkTheme ? 0.13 : 0.1)} 100%)`,
		'--brand-gradient-strong-bg': `linear-gradient(270deg, ${rgbaFromHex(strongColor, isDarkTheme ? 0.18 : 0.2)} 10%, ${rgbaFromHex(mixHexColors(normalizedAccentColor, isDarkTheme ? '#ffffff' : '#000000', isDarkTheme ? 0.38 : 0.2), isDarkTheme ? 0.12 : 0.16)} 100%)`,
		'--brand-gradient-button': isDarkTheme
			? 'rgba(255, 255, 255, 0.5)'
			: 'rgba(255, 255, 255, 0.08)',
		'--brand-gradient-border': rgbaFromHex(normalizedAccentColor, isDarkTheme ? 0.15 : 0.14),
		'--brand-gradient-fade-out-color': `linear-gradient(to bottom, ${rgbaFromHex(normalizedAccentColor, 0)} 0%, ${rgbaFromHex(fadedColor, isDarkTheme ? 0.18 : 0.12)} ${isDarkTheme ? 70 : 80}%)`,
		'--loading-bar-gradient': `linear-gradient(to right, var(--color-brand) 0%, ${loadingGradientEnd} 100%)`,
		'--color-button-bg-selected': isDarkTheme
			? 'var(--color-brand)'
			: 'var(--color-brand-highlight)',
		'--color-button-text-selected': isDarkTheme
			? 'var(--color-accent-contrast)'
			: 'var(--color-brand)',
	}
}
