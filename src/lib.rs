use dprint_core::plugins::FileMatchingInfo;
use dprint_core::plugins::FormatResult;
use dprint_core::plugins::PluginInfo;
use dprint_core::plugins::PluginResolveConfigurationResult;
use dprint_core::plugins::SyncFormatRequest;
use dprint_core::plugins::SyncHostFormatRequest;

use anyhow::Result;
use dprint_core::configuration::get_value;
use dprint_core::configuration::ConfigKeyMap;
use dprint_core::configuration::GlobalConfiguration;
#[cfg(target_arch = "wasm32")]
use dprint_core::generate_plugin_code;
use dprint_core::plugins::SyncPluginHandler;

pub mod configuration;

use configuration::Configuration;

#[derive(Default)]
pub struct TypstPluginHandler;

impl SyncPluginHandler<Configuration> for TypstPluginHandler {
    fn plugin_info(&mut self) -> PluginInfo {
        let version = env!("CARGO_PKG_VERSION").to_string();
        PluginInfo {
            name: env!("CARGO_PKG_NAME").to_string(),
            version: version.clone(),
            config_key: "typst".to_string(),
            help_url: "https://github.com/kachick/dprint-plugin-typstyle".to_string(),
            config_schema_url: format!(
                "https://plugins.dprint.dev/kachick/typstyle/{}/schema.json",
                version
            ),
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

        let line_width = get_value(
            &mut config,
            "lineWidth",
            global_config
                .line_width
                .unwrap_or(typstyle_core::Config::new().max_width as u32),
            &mut diagnostics,
        );

        let indent_width = get_value(
            &mut config,
            "indentWidth",
            global_config
                .indent_width
                .unwrap_or(typstyle_core::Config::new().tab_spaces as u8),
            &mut diagnostics,
        );

        let blank_lines_upper_bound = get_value(
            &mut config,
            "blankLinesUpperBound",
            typstyle_core::Config::new().blank_lines_upper_bound as u32,
            &mut diagnostics,
        );

        PluginResolveConfigurationResult {
            config: Configuration {
                line_width,
                indent_width,
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
            return Ok(None);
        }

        let text = String::from_utf8_lossy(&request.file_bytes);

        let config = typstyle_core::Config {
            tab_spaces: request.config.indent_width as usize,
            max_width: request.config.line_width as usize,
            blank_lines_upper_bound: request.config.blank_lines_upper_bound as usize,
            reorder_import_items: false,
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
