use dprint_core::configuration::{
    ConfigKeyMap, GlobalConfiguration, get_unknown_property_diagnostics, get_value,
};
use dprint_core::plugins::{
    FileMatchingInfo, FormatError, FormatResult, PluginInfo, PluginResolveConfigurationResult,
    SyncFormatRequest, SyncHostFormatRequest, SyncPluginHandler,
};

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
        let default_config = Configuration::default();

        let line_width = get_value(
            &mut config,
            "lineWidth",
            global_config
                .line_width
                .unwrap_or(default_config.line_width),
            &mut diagnostics,
        );

        let indent_width = get_value(
            &mut config,
            "indentWidth",
            global_config
                .indent_width
                .unwrap_or(default_config.indent_width),
            &mut diagnostics,
        );

        let blank_lines_upper_bound = get_value(
            &mut config,
            "blankLinesUpperBound",
            default_config.blank_lines_upper_bound,
            &mut diagnostics,
        );

        let reorder_import_items = get_value(
            &mut config,
            "reorderImportItems",
            default_config.reorder_import_items,
            &mut diagnostics,
        );

        let wrap_mode = get_value(
            &mut config,
            "wrapMode",
            default_config.wrap_mode,
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
                additive: false,
            },
        }
    }

    fn format(
        &mut self,
        request: SyncFormatRequest<Configuration>,
        _format_with_host: impl FnMut(SyncHostFormatRequest) -> FormatResult,
    ) -> FormatResult {
        let text = std::str::from_utf8(&request.file_bytes)?;

        let config = typstyle_core::Config::from(request.config);
        let formatter = typstyle_core::Typstyle::new(config);

        if let Some(range) = request.range {
            let mut start = range.start.min(text.len());
            let mut end = range.end.min(text.len());
            if start > end {
                start = end;
            }
            while !text.is_char_boundary(start) {
                start = start.saturating_sub(1);
            }
            while !text.is_char_boundary(end) {
                end = (end + 1).min(text.len());
            }

            let source = typst_syntax::Source::detached(text.to_string());
            let range_result = formatter
                .format_source_range(source, start..end)
                .map_err(FormatError::new)?;

            if range_result.source_range.start > text.len()
                || range_result.source_range.end > text.len()
                || range_result.source_range.start > range_result.source_range.end
            {
                return Ok(None);
            }

            let mut new_text = String::with_capacity(
                text.len().saturating_sub(range_result.source_range.len())
                    + range_result.content.len(),
            );
            new_text.push_str(&text[..range_result.source_range.start]);
            new_text.push_str(&range_result.content);
            new_text.push_str(&text[range_result.source_range.end..]);

            if new_text != text {
                Ok(Some(new_text.into_bytes()))
            } else {
                Ok(None)
            }
        } else {
            match formatter.format_text(text).render() {
                Ok(result) if result != text => Ok(Some(result.into())),
                Ok(_) => Ok(None),
                Err(err) => Err(FormatError::new(err)),
            }
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

// generate_plugin_code! initializes a static variable, so the second argument must be a const expression.
// Default::default() cannot be used here because trait methods cannot be called in statics.
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
        assert_eq!(result.config, Configuration::default());
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
    fn test_format_range() {
        let mut handler = TypstPluginHandler;
        let resolve_result =
            handler.resolve_config(ConfigKeyMap::new(), &GlobalConfiguration::default());
        let cancellation_token = NullCancellationToken;
        let original = "#set text(  size: 10pt  )\n#set text(  fill: red  )\n";
        let request = SyncFormatRequest {
            file_path: &PathBuf::from("test.typ"),
            file_bytes: original.as_bytes().to_vec(),
            config_id: FormatConfigId::from_raw(1),
            config: &resolve_result.config,
            range: Some(0..25),
            token: &cancellation_token,
        };
        let formatted = handler.format(request, |_| unreachable!()).unwrap();
        assert!(formatted.is_some());
        let formatted_str = String::from_utf8(formatted.unwrap()).unwrap();
        assert_eq!(
            formatted_str,
            "#set text(size: 10pt)\n#set text(  fill: red  )\n"
        );
    }

    #[test]
    fn test_format_range_already_formatted_returns_none() {
        let mut handler = TypstPluginHandler;
        let resolve_result =
            handler.resolve_config(ConfigKeyMap::new(), &GlobalConfiguration::default());
        let cancellation_token = NullCancellationToken;
        let request = SyncFormatRequest {
            file_path: &PathBuf::from("test.typ"),
            file_bytes: b"#set text(size: 10pt)\n".to_vec(),
            config_id: FormatConfigId::from_raw(1),
            config: &resolve_result.config,
            range: Some(0..5),
            token: &cancellation_token,
        };
        let formatted = handler.format(request, |_| unreachable!()).unwrap();
        assert_eq!(formatted, None);
    }

    #[test]
    fn test_format_range_out_of_bounds_returns_none() {
        let mut handler = TypstPluginHandler;
        let resolve_result =
            handler.resolve_config(ConfigKeyMap::new(), &GlobalConfiguration::default());
        let cancellation_token = NullCancellationToken;
        let request = SyncFormatRequest {
            file_path: &PathBuf::from("test.typ"),
            file_bytes: b"#set text(size: 10pt)\n".to_vec(),
            config_id: FormatConfigId::from_raw(1),
            config: &resolve_result.config,
            range: Some(100..200),
            token: &cancellation_token,
        };
        let formatted = handler.format(request, |_| unreachable!()).unwrap();
        assert_eq!(formatted, None);
    }

    #[test]
    fn test_format_range_syntax_error() {
        let mut handler = TypstPluginHandler;
        let resolve_result =
            handler.resolve_config(ConfigKeyMap::new(), &GlobalConfiguration::default());
        let cancellation_token = NullCancellationToken;
        let request = SyncFormatRequest {
            file_path: &PathBuf::from("test.typ"),
            file_bytes: b"#let x = (\n".to_vec(),
            config_id: FormatConfigId::from_raw(1),
            config: &resolve_result.config,
            range: Some(0..5),
            token: &cancellation_token,
        };
        let result = handler.format(request, |_| unreachable!());
        assert!(result.is_err());
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

#[cfg(test)]
mod spec_test;
