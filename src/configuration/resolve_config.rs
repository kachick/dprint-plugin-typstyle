use super::Configuration;
use super::WrapMode;
use dprint_core::configuration::*;

/// Resolves configuration from a collection of key value strings.
///
/// # Example
///
/// ```
/// use dprint_core::configuration::ConfigKeyMap;
/// use dprint_core::configuration::resolve_global_config;
/// use dprint_plugin_typstyle::configuration::resolve_config;
///
/// let mut config_map = ConfigKeyMap::new(); // get a collection of key value pairs from somewhere
/// let global_config_result = resolve_global_config(&mut config_map);
///
/// // check global_config_result.diagnostics here...
///
/// let config_map = ConfigKeyMap::new(); // get a collection of k/v pairs from somewhere
/// let config_result = resolve_config(
///     config_map,
///     &global_config_result.config
/// );
///
/// // check config_result.diagnostics here and use config_result.config
/// ```
pub fn resolve_config(
  config: ConfigKeyMap,
  global_config: &GlobalConfiguration,
) -> ResolveConfigurationResult<Configuration> {
  let mut diagnostics = Vec::new();
  let mut config = config;
  // default to the options of typstyle rather than dprint's recommended ones
  let defaults = typstyle_core::Config::new();

  let resolved_config = Configuration {
    line_width: get_value(
      &mut config,
      "lineWidth",
      global_config.line_width.unwrap_or(defaults.max_width as u32),
      &mut diagnostics,
    ),
    indent_width: get_value(
      &mut config,
      "indentWidth",
      global_config.indent_width.unwrap_or(defaults.tab_spaces as u8),
      &mut diagnostics,
    ),
    blank_lines_upper_bound: get_value(
      &mut config,
      "blankLinesUpperBound",
      defaults.blank_lines_upper_bound as u32,
      &mut diagnostics,
    ),
    reorder_import_items: get_value(
      &mut config,
      "reorderImportItems",
      defaults.reorder_import_items,
      &mut diagnostics,
    ),
    wrap_mode: get_value(&mut config, "wrapMode", WrapMode::default(), &mut diagnostics),
  };

  diagnostics.extend(get_unknown_property_diagnostics(config));

  ResolveConfigurationResult {
    config: resolved_config,
    diagnostics,
  }
}
