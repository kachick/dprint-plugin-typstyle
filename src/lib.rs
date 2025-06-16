use dprint_core::configuration::{ConfigKeyMap, GlobalConfiguration, get_value};
use dprint_core::plugins::{
    FileMatchingInfo, FormatResult, PluginInfo, PluginResolveConfigurationResult,
    SyncFormatRequest, SyncHostFormatRequest, SyncPluginHandler,
};

use anyhow::Result;

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
        let typestyle_defaults = typstyle_core::Config::new();

        let line_width = get_value(
            &mut config,
            "lineWidth",
            global_config
                .line_width
                .unwrap_or(typestyle_defaults.max_width as u32),
            &mut diagnostics,
        );

        let indent_width = get_value(
            &mut config,
            "indentWidth",
            global_config
                .indent_width
                .unwrap_or(typestyle_defaults.tab_spaces as u8),
            &mut diagnostics,
        );

        let blank_lines_upper_bound = get_value(
            &mut config,
            "blankLinesUpperBound",
            typestyle_defaults.blank_lines_upper_bound as u32,
            &mut diagnostics,
        );

        let reorder_import_items = get_value(
            &mut config,
            "reorderImportItems",
            typestyle_defaults.reorder_import_items,
            &mut diagnostics,
        );

        let wrap_text = get_value(
            &mut config,
            "wrapText",
            typestyle_defaults.wrap_text,
            &mut diagnostics,
        );

        PluginResolveConfigurationResult {
            config: Configuration {
                line_width,
                indent_width,
                blank_lines_upper_bound,
                reorder_import_items,
                wrap_text,
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
            reorder_import_items: request.config.reorder_import_items,
            wrap_text: request.config.wrap_text,
            collapse_markup_spaces: false, // https://github.com/Enter-tainer/typstyle/blob/655d66ca07adde88a8caa48a2d972ede784ff5d7/crates/typstyle-core/src/config.rs#L27
        };
        let formatter = typstyle_core::Typstyle::new(config);

        match formatter.format_text(text.as_ref()).render() {
            Ok(result) if result != text => Ok(Some(result.into())),
            Ok(_) => Ok(None),
            Err(err) => Err(anyhow::anyhow!("Formatting failed: {}", err)),
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
use dprint_core::generate_plugin_code;

#[cfg(target_arch = "wasm32")]
generate_plugin_code!(TypstPluginHandler, TypstPluginHandler, Configuration);
