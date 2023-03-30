mod cli;
mod config;
#[cfg(test)]
mod tests;

use std::path::PathBuf;

use crate::{
    color,
    keybindings::{self, Keybindings},
};

use serde::Deserialize;

pub use self::cli::Args;
pub use self::config::Config;
pub use self::config::FontOptions;

#[derive(Deserialize, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ThemeType {
    #[default]
    Auto,
    Dark,
    Light,
}

#[derive(Deserialize, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ResolvedTheme {
    Dark,
    #[default]
    Light,
}

impl ResolvedTheme {
    fn detect() -> Self {
        match dark_light::detect() {
            dark_light::Mode::Default => Self::default(),
            dark_light::Mode::Dark => Self::Dark,
            dark_light::Mode::Light => Self::Light,
        }
    }
}

impl From<ThemeType> for ResolvedTheme {
    fn from(theme_ty: ThemeType) -> Self {
        match theme_ty {
            ThemeType::Auto => Self::detect(),
            ThemeType::Dark => Self::Dark,
            ThemeType::Light => Self::Light,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Opts {
    pub file_path: PathBuf,
    pub theme: color::Theme,
    pub scale: Option<f32>,
    pub lines_to_scroll: f32,
    pub font_opts: FontOptions,
    pub keybindings: Keybindings,
}

impl Opts {
    pub fn parse_and_load_from(args: &Args, config: config::Config) -> Self {
        let config::Config {
            theme: config_theme,
            scale: config_scale,
            lines_to_scroll: config_lines_to_scroll,
            light_theme: config_light_theme,
            dark_theme: config_dark_theme,
            font_options: config_font_options,
            keybindings:
                config::KeybindingsSection {
                    base: keybindings_base,
                    extra: keybindings_extra,
                },
        } = config;

        let resolved_theme = ResolvedTheme::from(args.theme.unwrap_or(config_theme));
        let theme = match resolved_theme {
            ResolvedTheme::Dark => match config_dark_theme {
                Some(config_dark_theme) => config_dark_theme.merge(color::DARK_DEFAULT),
                None => color::DARK_DEFAULT,
            },
            ResolvedTheme::Light => match config_light_theme {
                Some(config_light_theme) => config_light_theme.merge(color::LIGHT_DEFAULT),
                None => color::LIGHT_DEFAULT,
            },
        };

        let font_opts = config_font_options.unwrap_or_default();

        let keybindings = {
            let mut temp = keybindings_base.unwrap_or_else(keybindings::defaults);
            if let Some(extra) = keybindings_extra {
                temp.extend(extra.into_iter());
            }
            temp
        };

        Self {
            file_path: args.file_path.clone(),
            theme,
            scale: args.scale.or(config_scale),
            lines_to_scroll: config_lines_to_scroll.0,
            font_opts,
            keybindings,
        }
    }
}
