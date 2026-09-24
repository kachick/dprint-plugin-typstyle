use dprint_core::configuration::{
    ConfigKeyMap, GlobalConfiguration, get_unknown_property_diagnostics, get_value,
};
use dprint_core::plugins::{
    FileMatchingInfo, FormatError, FormatResult, PluginInfo, PluginResolveConfigurationResult,
    SyncFormatRequest, SyncHostFormatRequest, SyncPluginHandler,
};

pub mod configuration;
use configuration::{Configuration, WrapMode};

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
                "https://plugins.dprint.dev/kachick/typstyle/{version}/schema.json"
            ),
            update_url: Some("https://plugins.dprint.dev/kachick/typstyle/latest.json".to_string()),
        }
    }

    fn license_text(&mut self) -> String {
        include_str!("../LICENSE").to_string()
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

        let wrap_mode = get_value(
            &mut config,
            "wrapMode",
            WrapMode::default(),
            &mut diagnostics,
        );

        diagnostics.extend(get_unknown_property_diagnostics(config));

        PluginResolveConfigurationResult {
            config: Configuration {
                line_width,
                indent_width,
                blank_lines_upper_bound,
                reorder_import_items,
                wrap_mode,
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

        let text = std::str::from_utf8(&request.file_bytes)?;

        let config = typstyle_core::Config::from(request.config);
        let formatter = typstyle_core::Typstyle::new(config);

        match formatter.format_text(text).render() {
            Ok(result) if result != text => Ok(Some(result.into())),
            Ok(_) => Ok(None),
            Err(err) => Err(FormatError::new(err)),
        }
    }

    fn check_config_updates(
        &self,
        _message: dprint_core::plugins::CheckConfigUpdatesMessage,
    ) -> Result<Vec<dprint_core::plugins::ConfigChange>, FormatError> {
        Ok(Vec::new())
    }
}

#[cfg(target_arch = "wasm32")]
use dprint_core::generate_plugin_code;

#[cfg(target_arch = "wasm32")]
generate_plugin_code!(TypstPluginHandler, TypstPluginHandler);

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use dprint_core::configuration::ConfigKeyValue;
    use dprint_core::plugins::{FormatConfigId, NullCancellationToken};

    use super::*;

    #[test]
    fn test_resolve_config_defaults() {
        let mut handler = TypstPluginHandler;
        let result = handler.resolve_config(ConfigKeyMap::new(), &GlobalConfiguration::default());
        assert!(result.diagnostics.is_empty());
        assert_eq!(result.config.line_width, 80);
        assert_eq!(result.config.indent_width, 2);
        assert_eq!(result.config.blank_lines_upper_bound, 1);
        assert!(result.config.reorder_import_items);
        assert_eq!(result.config.wrap_mode, WrapMode::None);
        assert_eq!(result.file_matching.file_extensions, vec!["typ"]);
    }

    #[test]
    fn test_resolve_config_with_global() {
        let mut handler = TypstPluginHandler;
        let global = GlobalConfiguration {
            line_width: Some(100),
            indent_width: Some(4),
            ..Default::default()
        };
        let result = handler.resolve_config(ConfigKeyMap::new(), &global);
        assert!(result.diagnostics.is_empty());
        assert_eq!(result.config.line_width, 100);
        assert_eq!(result.config.indent_width, 4);
    }

    #[test]
    fn test_resolve_config_unknown_property() {
        let mut handler = TypstPluginHandler;
        let mut config = ConfigKeyMap::new();
        config.insert(
            "unknownProp".to_string(),
            ConfigKeyValue::String("val".to_string()),
        );
        let result = handler.resolve_config(config, &GlobalConfiguration::default());
        assert_eq!(result.diagnostics.len(), 1);
        assert_eq!(result.diagnostics[0].property_name, "unknownProp");
    }

    #[test]
    fn test_format() {
        let mut handler = TypstPluginHandler;
        let resolve_result =
            handler.resolve_config(ConfigKeyMap::new(), &GlobalConfiguration::default());
        let cancellation_token = NullCancellationToken;
        let request = SyncFormatRequest {
            file_path: &PathBuf::from("test.typ"),
            file_bytes: b"#set text(  size: 10pt  )".to_vec(),
            config_id: FormatConfigId::from_raw(1),
            config: &resolve_result.config,
            range: None,
            token: &cancellation_token,
        };
        let formatted = handler.format(request, |_| unreachable!()).unwrap();
        assert!(formatted.is_some());
        let formatted_str = String::from_utf8(formatted.unwrap()).unwrap();
        assert_eq!(formatted_str, "#set text(size: 10pt)\n");
    }

    #[test]
    fn test_format_range_returns_none() {
        let mut handler = TypstPluginHandler;
        let resolve_result =
            handler.resolve_config(ConfigKeyMap::new(), &GlobalConfiguration::default());
        let cancellation_token = NullCancellationToken;
        let request = SyncFormatRequest {
            file_path: &PathBuf::from("test.typ"),
            file_bytes: b"#set text(size: 10pt)\n".to_vec(),
            config_id: FormatConfigId::from_raw(1),
            config: &resolve_result.config,
            range: Some(std::ops::Range { start: 0, end: 5 }),
            token: &cancellation_token,
        };
        let formatted = handler.format(request, |_| unreachable!()).unwrap();
        assert_eq!(formatted, None);
    }

    #[test]
    fn test_format_invalid_utf8() {
        let mut handler = TypstPluginHandler;
        let resolve_result =
            handler.resolve_config(ConfigKeyMap::new(), &GlobalConfiguration::default());
        let cancellation_token = NullCancellationToken;
        let request = SyncFormatRequest {
            file_path: &PathBuf::from("test.typ"),
            file_bytes: vec![0xFF, 0xFE, 0xFD],
            config_id: FormatConfigId::from_raw(1),
            config: &resolve_result.config,
            range: None,
            token: &cancellation_token,
        };
        let result = handler.format(request, |_| unreachable!());
        assert!(result.is_err());
    }
}
