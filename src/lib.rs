use dprint_core::plugins::FileMatchingInfo;
use dprint_core::plugins::FormatResult;
use dprint_core::plugins::PluginInfo;
use dprint_core::plugins::PluginResolveConfigurationResult;
use dprint_core::plugins::SyncFormatRequest;
use dprint_core::plugins::SyncHostFormatRequest;
use serde::Serialize;

use anyhow::Result;
use dprint_core::configuration::get_value;
use dprint_core::configuration::ConfigKeyMap;
use dprint_core::configuration::GlobalConfiguration;
#[cfg(target_arch = "wasm32")]
use dprint_core::generate_plugin_code;
use dprint_core::plugins::SyncPluginHandler;

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Configuration {
    // List of upstream: https://github.com/Enter-tainer/typstyle/blob/v0.12.14/crates/typstyle-core/src/config.rs#L4-L11
    pub tab_spaces: u8,
    pub max_width: u32,
    pub blank_lines_upper_bound: u32,
}

#[derive(Default)]
pub struct TypstPluginHandler;

impl SyncPluginHandler<Configuration> for TypstPluginHandler {
    fn plugin_info(&mut self) -> PluginInfo {
        PluginInfo {
            name: env!("CARGO_PKG_NAME").to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            config_key: "typst".to_string(),
            help_url: "https://github.com/kachick/dprint-plugin-typstyle".to_string(), // fill this in
            config_schema_url: "".to_string(), // leave this empty for now
            update_url: Some("https://plugins.dprint.dev/kachick/typstyle/latest.json".to_string()),
        }
    }

    fn license_text(&mut self) -> String {
        std::str::from_utf8(include_bytes!("../LICENSE"))
            .unwrap()
            .into()
    }

    fn resolve_config(
        &mut self,
        config: ConfigKeyMap,
        global_config: &GlobalConfiguration,
    ) -> PluginResolveConfigurationResult<Configuration> {
        let mut config = config;
        let mut diagnostics = Vec::new();
        let tab_spaces = get_value(
            &mut config,
            "tab_spaces",
            global_config
                .indent_width
                .unwrap_or(typstyle_core::Config::new().tab_spaces as u8),
            &mut diagnostics,
        );

        let max_width = get_value(
            &mut config,
            "column", // Use different name in actual dprint config file to keep backward compatibility. It might be changed in future major/minor version updating.
            global_config
                .line_width
                .unwrap_or(typstyle_core::Config::new().max_width as u32),
            &mut diagnostics,
        );

        let blank_lines_upper_bound = get_value(
            &mut config,
            "blank_lines_upper_bound",
            typstyle_core::Config::new().blank_lines_upper_bound as u32,
            &mut diagnostics,
        );

        PluginResolveConfigurationResult {
            config: Configuration {
                tab_spaces,
                max_width,
                blank_lines_upper_bound,
            },
            diagnostics,
            file_matching: FileMatchingInfo {
                file_extensions: vec!["typ".to_string()],
                file_names: vec![],
            },
        }
    }

    fn format(
        &mut self,
        request: SyncFormatRequest<Configuration>,
        _format_with_host: impl FnMut(SyncHostFormatRequest) -> FormatResult,
    ) -> FormatResult {
        if request.range.is_some() {
            return Ok(None); // not implemented
        }

        let text = String::from_utf8_lossy(&request.file_bytes);

        let config = typstyle_core::Config {
            tab_spaces: request.config.tab_spaces as usize,
            max_width: request.config.max_width as usize,
            blank_lines_upper_bound: request.config.blank_lines_upper_bound as usize,
        };
        let formatter = typstyle_core::Typstyle::new(config);

        let result = formatter
            .format_content(text.as_ref())
            .unwrap_or_else(|_| text.to_string());

        if result == text {
            Ok(None)
        } else {
            Ok(Some(result.into()))
        }
    }

    fn check_config_updates(
        &self,
        _message: dprint_core::plugins::CheckConfigUpdatesMessage,
    ) -> Result<Vec<dprint_core::plugins::ConfigChange>> {
        Ok(Vec::new())
    }
}

#[cfg(target_arch = "wasm32")]
generate_plugin_code!(TypstPluginHandler, TypstPluginHandler, Configuration);
