//! Theseus settings file

use serde::{Deserialize, Serialize};
use sqlx::{Pool, Row, Sqlite};
use std::collections::HashMap;

// Types
/// Global Theseus settings
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Settings {
    pub max_concurrent_downloads: usize,
    pub max_concurrent_writes: usize,

    pub theme: Theme,
    pub accent_color: String,
    pub locale: String,
    pub default_page: DefaultPage,
    pub collapsed_navigation: bool,
    pub hide_nametag_skins_page: bool,
    pub advanced_rendering: bool,
    pub native_decorations: bool,
    pub toggle_sidebar: bool,

    pub telemetry: bool,
    pub discord_rpc: bool,
    pub personalized_ads: bool,

    pub onboarded: bool,

    pub extra_launch_args: Vec<String>,
    pub custom_env_vars: Vec<(String, String)>,
    pub memory: MemorySettings,
    pub force_fullscreen: bool,
    pub game_resolution: WindowSize,
    pub hide_on_process_start: bool,
    pub hooks: Hooks,

    pub custom_dir: Option<String>,
    pub prev_custom_dir: Option<String>,
    pub migrated: bool,

    pub developer_mode: bool,
    pub feature_flags: HashMap<FeatureFlag, bool>,

    pub skipped_update: Option<String>,
    pub pending_update_toast_for_version: Option<String>,
    pub auto_download_updates: Option<bool>,

    pub version: usize,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Eq, Hash, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum FeatureFlag {
    PagePath,
    ProjectBackground,
    WorldsTab,
    WorldsInHome,
    ServersInApp,
    ServerProjectQa,
    I18nDebug,
}

impl Settings {
    const CURRENT_VERSION: usize = 2;

    pub async fn get(
        exec: impl sqlx::Executor<'_, Database = sqlx::Sqlite>,
    ) -> crate::Result<Self> {
        let res = sqlx::query(
            "
            SELECT
                max_concurrent_writes, max_concurrent_downloads,
                theme, accent_color, locale, default_page, collapsed_navigation, hide_nametag_skins_page, advanced_rendering, native_decorations,
                discord_rpc, developer_mode, telemetry, personalized_ads,
                onboarded,
                json(extra_launch_args) extra_launch_args, json(custom_env_vars) custom_env_vars,
                mc_memory_max, mc_force_fullscreen, mc_game_resolution_x, mc_game_resolution_y, hide_on_process_start,
                hook_pre_launch, hook_wrapper, hook_post_exit,
                custom_dir, prev_custom_dir, migrated, json(feature_flags) feature_flags, toggle_sidebar,
                skipped_update, pending_update_toast_for_version, auto_download_updates,
                version
            FROM settings
            "
        )
            .fetch_one(exec)
            .await?;

        Ok(Self {
            max_concurrent_downloads: res
                .get::<i64, _>("max_concurrent_downloads")
                as usize,
            max_concurrent_writes: res.get::<i64, _>("max_concurrent_writes")
                as usize,
            theme: Theme::from_string(&res.get::<String, _>("theme")),
            accent_color: res.get::<String, _>("accent_color"),
            locale: res.get::<String, _>("locale"),
            default_page: DefaultPage::from_string(
                &res.get::<String, _>("default_page"),
            ),
            collapsed_navigation: res.get::<i64, _>("collapsed_navigation")
                == 1,
            hide_nametag_skins_page: res
                .get::<i64, _>("hide_nametag_skins_page")
                == 1,
            advanced_rendering: res.get::<i64, _>("advanced_rendering") == 1,
            native_decorations: res.get::<i64, _>("native_decorations") == 1,
            toggle_sidebar: res.get::<i64, _>("toggle_sidebar") == 1,
            telemetry: res.get::<i64, _>("telemetry") == 1,
            discord_rpc: res.get::<i64, _>("discord_rpc") == 1,
            developer_mode: res.get::<i64, _>("developer_mode") == 1,
            personalized_ads: res.get::<i64, _>("personalized_ads") == 1,
            onboarded: res.get::<i64, _>("onboarded") == 1,
            extra_launch_args: res
                .get::<Option<String>, _>("extra_launch_args")
                .as_ref()
                .and_then(|x| serde_json::from_str(x).ok())
                .unwrap_or_default(),
            custom_env_vars: res
                .get::<Option<String>, _>("custom_env_vars")
                .as_ref()
                .and_then(|x| serde_json::from_str(x).ok())
                .unwrap_or_default(),
            memory: MemorySettings {
                maximum: res.get::<i64, _>("mc_memory_max") as u32,
            },
            force_fullscreen: res.get::<i64, _>("mc_force_fullscreen") == 1,
            game_resolution: WindowSize(
                res.get::<i64, _>("mc_game_resolution_x") as u16,
                res.get::<i64, _>("mc_game_resolution_y") as u16,
            ),
            hide_on_process_start: res.get::<i64, _>("hide_on_process_start")
                == 1,
            hooks: Hooks {
                pre_launch: res.get::<Option<String>, _>("hook_pre_launch"),
                wrapper: res.get::<Option<String>, _>("hook_wrapper"),
                post_exit: res.get::<Option<String>, _>("hook_post_exit"),
            },
            custom_dir: res.get::<Option<String>, _>("custom_dir"),
            prev_custom_dir: res.get::<Option<String>, _>("prev_custom_dir"),
            migrated: res.get::<i64, _>("migrated") == 1,
            feature_flags: res
                .get::<Option<String>, _>("feature_flags")
                .as_ref()
                .and_then(|x| serde_json::from_str(x).ok())
                .unwrap_or_default(),
            skipped_update: res.get::<Option<String>, _>("skipped_update"),
            pending_update_toast_for_version: res
                .get::<Option<String>, _>("pending_update_toast_for_version"),
            auto_download_updates: res
                .get::<Option<i64>, _>("auto_download_updates")
                .map(|x| x == 1),
            version: res.get::<i64, _>("version") as usize,
        })
    }

    pub async fn update(
        &self,
        exec: impl sqlx::Executor<'_, Database = sqlx::Sqlite>,
    ) -> crate::Result<()> {
        let max_concurrent_writes = self.max_concurrent_writes as i32;
        let max_concurrent_downloads = self.max_concurrent_downloads as i32;
        let theme = self.theme.as_str();
        let default_page = self.default_page.as_str();
        let extra_launch_args = serde_json::to_string(&self.extra_launch_args)?;
        let custom_env_vars = serde_json::to_string(&self.custom_env_vars)?;
        let feature_flags = serde_json::to_string(&self.feature_flags)?;
        let version = self.version as i64;

        sqlx::query(
            "
            UPDATE settings
            SET
                max_concurrent_writes = $1,
                max_concurrent_downloads = $2,

                theme = $3,
                accent_color = $4,
                locale = $5,
                default_page = $6,
                collapsed_navigation = $7,
                advanced_rendering = $8,
                native_decorations = $9,

                discord_rpc = $10,
                developer_mode = $11,
                telemetry = $12,
                personalized_ads = $13,

                onboarded = $14,

                extra_launch_args = jsonb($15),
                custom_env_vars = jsonb($16),
                mc_memory_max = $17,
                mc_force_fullscreen = $18,
                mc_game_resolution_x = $19,
                mc_game_resolution_y = $20,
                hide_on_process_start = $21,

                hook_pre_launch = $22,
                hook_wrapper = $23,
                hook_post_exit = $24,

                custom_dir = $25,
                prev_custom_dir = $26,
                migrated = $27,

                toggle_sidebar = $28,
                feature_flags = $29,
                hide_nametag_skins_page = $30,

                skipped_update = $31,
                pending_update_toast_for_version = $32,
                auto_download_updates = $33,

                version = $34
            ",
        )
        .bind(max_concurrent_writes)
        .bind(max_concurrent_downloads)
        .bind(theme)
        .bind(&self.accent_color)
        .bind(&self.locale)
        .bind(default_page)
        .bind(self.collapsed_navigation)
        .bind(self.advanced_rendering)
        .bind(self.native_decorations)
        .bind(self.discord_rpc)
        .bind(self.developer_mode)
        .bind(self.telemetry)
        .bind(self.personalized_ads)
        .bind(self.onboarded)
        .bind(extra_launch_args)
        .bind(custom_env_vars)
        .bind(self.memory.maximum)
        .bind(self.force_fullscreen)
        .bind(self.game_resolution.0)
        .bind(self.game_resolution.1)
        .bind(self.hide_on_process_start)
        .bind(&self.hooks.pre_launch)
        .bind(&self.hooks.wrapper)
        .bind(&self.hooks.post_exit)
        .bind(&self.custom_dir)
        .bind(&self.prev_custom_dir)
        .bind(self.migrated)
        .bind(self.toggle_sidebar)
        .bind(feature_flags)
        .bind(self.hide_nametag_skins_page)
        .bind(&self.skipped_update)
        .bind(&self.pending_update_toast_for_version)
        .bind(
            self.auto_download_updates
                .map(|value| if value { 1 } else { 0 }),
        )
        .bind(version)
        .execute(exec)
        .await?;

        Ok(())
    }

    pub async fn migrate(exec: &Pool<Sqlite>) -> crate::Result<()> {
        let mut settings = Self::get(exec).await?;

        if settings.version < Settings::CURRENT_VERSION {
            tracing::info!(
                "Migrating settings version {} to {:?}",
                settings.version,
                Settings::CURRENT_VERSION
            );
        }
        while settings.version < Settings::CURRENT_VERSION {
            if let Err(err) = settings.perform_migration() {
                tracing::error!(
                    "Failed to migrate settings from version {}: {}",
                    settings.version,
                    err
                );
                return Err(err);
            }
        }

        settings.update(exec).await?;

        Ok(())
    }

    pub fn perform_migration(&mut self) -> crate::Result<()> {
        match self.version {
            1 => {
                let quoter = shlex::Quoter::new().allow_nul(true);

                // Previously split by spaces
                if let Some(pre_launch) = self.hooks.pre_launch.as_ref() {
                    self.hooks.pre_launch =
                        Some(quoter.join(pre_launch.split(' ')).unwrap())
                }

                // Previously treated as complete path to command
                if let Some(wrapper) = self.hooks.wrapper.as_ref() {
                    self.hooks.wrapper =
                        Some(quoter.quote(wrapper).unwrap().to_string())
                }

                // Previously split by spaces
                if let Some(post_exit) = self.hooks.post_exit.as_ref() {
                    self.hooks.post_exit =
                        Some(quoter.join(post_exit.split(' ')).unwrap())
                }

                self.version = 2;
            }
            version => {
                return Err(crate::ErrorKind::OtherError(format!(
                    "Invalid settings version: {version}"
                ))
                .into());
            }
        }

        Ok(())
    }
}

/// Theseus theme
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Theme {
    Dark,
    Light,
    Oled,
    System,
}

impl Theme {
    pub fn as_str(&self) -> &'static str {
        match self {
            Theme::Dark => "dark",
            Theme::Light => "light",
            Theme::Oled => "oled",
            Theme::System => "system",
        }
    }

    pub fn from_string(string: &str) -> Theme {
        match string {
            "dark" => Theme::Dark,
            "light" => Theme::Light,
            "oled" => Theme::Oled,
            "system" => Theme::System,
            _ => Theme::Dark,
        }
    }
}

/// Minecraft memory settings
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct MemorySettings {
    pub maximum: u32,
}

/// Game window size
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct WindowSize(pub u16, pub u16);

/// Game initialization hooks
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde_with::serde_as]
pub struct Hooks {
    #[serde_as(as = "serde_with::NoneAsEmptyString")]
    pub pre_launch: Option<String>,
    #[serde_as(as = "serde_with::NoneAsEmptyString")]
    pub wrapper: Option<String>,
    #[serde_as(as = "serde_with::NoneAsEmptyString")]
    pub post_exit: Option<String>,
}

/// Opening window to start with
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum DefaultPage {
    Home,
    Library,
}

impl DefaultPage {
    pub fn as_str(&self) -> &'static str {
        match self {
            DefaultPage::Home => "home",
            DefaultPage::Library => "library",
        }
    }

    pub fn from_string(string: &str) -> Self {
        match string {
            "home" => Self::Home,
            "library" => Self::Library,
            _ => Self::Home,
        }
    }
}
