<script setup lang="ts">
import type { Labrinth } from '@modrinth/api-client'
import { CheckIcon, DownloadIcon, SpinnerIcon } from '@modrinth/assets'
import {
	Admonition,
	Button,
	ButtonStyled,
	defineMessages,
	injectModrinthClient,
	injectNotificationManager,
	LoadingIndicator,
	NavTabs,
	ProjectCard,
	ProjectCardList,
	useVIntl,
} from '@modrinth/ui'
import { computed, onMounted, ref } from 'vue'
import { useRoute, useRouter } from 'vue-router'

import { injectContentInstall } from '@/providers/content-install'
import { useBreadcrumbs } from '@/store/breadcrumbs.js'

const route = useRoute()
const router = useRouter()
const breadcrumbs = useBreadcrumbs()
breadcrumbs.setRootContext({ name: 'JS Gaming', link: route.path })

const client = injectModrinthClient()
const { handleError } = injectNotificationManager()
const { formatMessage } = useVIntl()
const { install: installContent } = injectContentInstall()

const messages = defineMessages({
	pageTitle: {
		id: 'app.jsgaming.title',
		defaultMessage: 'JS Gaming',
	},
	intro: {
		id: 'app.jsgaming.intro',
		defaultMessage: 'Optimierte Modpacks und Ressourcepacks für ein besseres Minecraft-Erlebnis.',
	},
	openOrganization: {
		id: 'app.jsgaming.open-organization',
		defaultMessage: 'JumpStone-Gaming auf Modrinth öffnen',
	},
	all: {
		id: 'app.jsgaming.tabs.all',
		defaultMessage: 'Alle',
	},
	modpacks: {
		id: 'app.jsgaming.tabs.modpacks',
		defaultMessage: 'Modpacks',
	},
	resourcepacks: {
		id: 'app.jsgaming.tabs.resourcepacks',
		defaultMessage: 'Ressourcepacks',
	},
	loading: {
		id: 'app.jsgaming.loading',
		defaultMessage: 'Projekte werden geladen...',
	},
	loadFailed: {
		id: 'app.jsgaming.load-failed',
		defaultMessage: 'Projekte konnten nicht geladen werden.',
	},
	empty: {
		id: 'app.jsgaming.empty',
		defaultMessage: 'In dieser Kategorie wurden keine Projekte gefunden.',
	},
	retry: {
		id: 'app.jsgaming.retry',
		defaultMessage: 'Erneut versuchen',
	},
	install: {
		id: 'app.jsgaming.install',
		defaultMessage: 'Installieren',
	},
	installing: {
		id: 'app.jsgaming.installing',
		defaultMessage: 'Installing...',
	},
	installed: {
		id: 'app.jsgaming.installed',
		defaultMessage: 'Installed',
	},
})

const organizationUrl = 'https://modrinth.com/organization/jumpstone-gaming'
const organizationCandidates = ['jumpstone-gaming', 'qrldrfiD']

const projects = ref<Labrinth.Projects.v3.Project[]>([])
const isLoading = ref(true)
const loadError = ref(false)
const installingProjects = ref<string[]>([])
const newlyInstalled = ref<string[]>([])

const tabLinks = computed(() => [
	{ label: formatMessage(messages.all), href: '' },
	{ label: formatMessage(messages.modpacks), href: 'modpacks' },
	{ label: formatMessage(messages.resourcepacks), href: 'resourcepacks' },
])

const visibleProjects = computed<Labrinth.Projects.v3.Project[]>(() =>
	[...projects.value]
		.filter(
			(project) =>
				project.project_types.includes('modpack') || project.project_types.includes('resourcepack'),
		)
		.sort((left, right) => {
			const leftType = left.project_types.includes('modpack') ? 'modpack' : 'resourcepack'
			const rightType = right.project_types.includes('modpack') ? 'modpack' : 'resourcepack'
			const typeOrder = leftType === rightType ? 0 : leftType === 'modpack' ? -1 : 1
			if (typeOrder !== 0) return typeOrder
			return left.name.localeCompare(right.name, undefined, { sensitivity: 'base' })
		}),
)

const selectedTab = computed(() => {
	const tab = route.query.tab
	if (tab === 'modpacks' || tab === 'resourcepacks') return tab
	return 'all'
})

const filteredProjects = computed<Labrinth.Projects.v3.Project[]>(() => {
	if (selectedTab.value === 'modpacks') {
		return visibleProjects.value.filter((project) => project.project_types.includes('modpack'))
	}
	if (selectedTab.value === 'resourcepacks') {
		return visibleProjects.value.filter((project) => project.project_types.includes('resourcepack'))
	}
	return visibleProjects.value
})

function isProjectInstalling(projectId: string) {
	return installingProjects.value.includes(projectId)
}

function isProjectInstalled(projectId: string) {
	return newlyInstalled.value.includes(projectId)
}

async function installProject(project: Labrinth.Projects.v3.Project) {
	if (isProjectInstalling(project.id) || isProjectInstalled(project.id)) return

	installingProjects.value.push(project.id)

	await installContent(
		project.id,
		null,
		null,
		'JsGaming',
		(versionId) => {
			if (versionId && !newlyInstalled.value.includes(project.id)) {
				newlyInstalled.value.push(project.id)
			}
		},
		(profile) => {
			router.push(`/instance/${profile}`)
		},
	).catch(handleError)

	installingProjects.value = installingProjects.value.filter((id) => id !== project.id)
}

async function loadProjects() {
	isLoading.value = true
	loadError.value = false

	try {
		let lastError: unknown

		for (const organizationId of organizationCandidates) {
			try {
				projects.value = await client.labrinth.organizations_v3.getProjects(organizationId)
				return
			} catch (error) {
				lastError = error
			}
		}

		throw lastError ?? new Error(formatMessage(messages.loadFailed))
	} catch (error) {
		const normalizedError: Error =
			error instanceof Error ? error : new Error(formatMessage(messages.loadFailed))
		handleError(normalizedError)
		projects.value = []
		loadError.value = true
	} finally {
		isLoading.value = false
	}
}

onMounted(() => {
	void loadProjects()
})
</script>

<template>
	<div class="p-6 flex flex-col gap-6">
		<section class="rounded-3xl border border-surface-4 bg-bg-raised p-6 flex flex-col gap-4">
			<div class="flex flex-col gap-2">
				<h1 class="m-0 text-3xl font-extrabold text-contrast">
					{{ formatMessage(messages.pageTitle) }}
				</h1>
				<p class="m-0 max-w-3xl text-secondary leading-relaxed">
					{{ formatMessage(messages.intro) }}
				</p>
			</div>
			<div class="flex flex-wrap items-center gap-3">
				<Button
					color="primary"
					large
					link="https://modrinth.com/organization/jumpstone-gaming"
					external
				>
					{{ formatMessage(messages.openOrganization) }}
				</Button>
			</div>
		</section>

		<NavTabs :links="tabLinks" query="tab" />

		<LoadingIndicator v-if="isLoading" />

		<Admonition v-else-if="loadError" type="critical" :header="formatMessage(messages.loadFailed)">
			<div class="flex flex-col gap-3">
				<p class="m-0">{{ formatMessage(messages.loadFailed) }}</p>
				<Button color="primary" :action="loadProjects">
					{{ formatMessage(messages.retry) }}
				</Button>
			</div>
		</Admonition>

		<Admonition
			v-else-if="filteredProjects.length === 0"
			type="info"
			:header="formatMessage(messages.empty)"
		>
		</Admonition>

		<ProjectCardList v-else layout="list">
			<ProjectCard
				v-for="project in filteredProjects"
				:key="project.id"
				:link="`/project/${project.slug || project.id}`"
				:icon-url="project.icon_url || undefined"
				:title="project.name"
				:author="{ name: 'JumpStone Gaming', link: organizationUrl }"
				:summary="project.summary"
				:downloads="project.downloads"
				:followers="project.followers"
				:date-published="project.published"
				:date-updated="project.updated"
				:tags="project.categories"
				layout="list"
			>
				<template #actions>
					<ButtonStyled color="brand" type="outlined">
						<button
							:disabled="isProjectInstalled(project.id) || isProjectInstalling(project.id)"
							class="shrink-0 no-wrap"
							@click.stop="installProject(project)"
						>
							<SpinnerIcon v-if="isProjectInstalling(project.id)" class="animate-spin" />
							<CheckIcon v-else-if="isProjectInstalled(project.id)" />
							<DownloadIcon v-else />
							{{
								formatMessage(
									isProjectInstalling(project.id)
										? messages.installing
										: isProjectInstalled(project.id)
											? messages.installed
											: messages.install,
								)
							}}
						</button>
					</ButtonStyled>
				</template>
			</ProjectCard>
		</ProjectCardList>
	</div>
</template>
