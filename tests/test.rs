extern crate dprint_development;
extern crate dprint_plugin_typstyle;

use std::path::PathBuf;
use std::sync::Arc;

use dprint_core::configuration::*;
use dprint_development::*;
use dprint_plugin_typstyle::configuration::Configuration;
use dprint_plugin_typstyle::configuration::resolve_config;
use dprint_plugin_typstyle::*;

#[test]
fn test_specs() {
  let global_config = GlobalConfiguration::default();

  run_specs(
    &PathBuf::from("./tests/specs"),
    &ParseSpecOptions {
      default_file_name: "file.typ",
    },
    &RunSpecsOptions {
      fix_failures: false,
      format_twice: true,
    },
    {
      let global_config = global_config.clone();
      Arc::new(move |file_path, file_text, range, spec_config| {
        let spec_config: ConfigKeyMap = serde_json::from_value(spec_config.clone().into()).unwrap();
        let config_result = resolve_config(spec_config, &global_config);
        ensure_no_diagnostics(&config_result.diagnostics);

        match range {
          Some(range) => format_text_range(file_path, file_text, range, &config_result.config),
          None => format_text(file_path, file_text, &config_result.config),
        }
        .map_err(|err| err.into())
      })
    },
    Arc::new(move |_file_path, _file_text, _spec_config| panic!("Plugin does not support dprint-core tracing.")),
  )
}

#[test]
fn should_have_schema_defaults_matching_resolved_config() {
  let schema: serde_json::Value = serde_json::from_str(include_str!("../deployment/schema.json")).unwrap();
  let properties = schema["properties"].as_object().unwrap();
  let config = serde_json::to_value(default_config()).unwrap();
  let config = config.as_object().unwrap();
  assert_eq!(properties.len(), config.len());
  for (key, value) in config {
    assert_eq!(&properties[key]["default"], value, "default of {key}");
  }
}

#[test]
fn should_use_global_config() {
  let global_config = GlobalConfiguration {
    line_width: Some(100),
    indent_width: Some(4),
    ..Default::default()
  };
  let result = resolve_config(Default::default(), &global_config);
  assert!(result.diagnostics.is_empty());
  assert_eq!(result.config.line_width, 100);
  assert_eq!(result.config.indent_width, 4);
}

#[test]
fn should_error_on_invalid_wrap_mode() {
  let mut config = ConfigKeyMap::new();
  config.insert("wrapMode".to_string(), ConfigKeyValue::String("unknown".to_string()));
  let result = resolve_config(config, &GlobalConfiguration::default());
  assert_eq!(result.diagnostics.len(), 1);
  assert_eq!(result.diagnostics[0].property_name, "wrapMode");
}

#[test]
fn should_fail_on_syntax_error() {
  let file_path = PathBuf::from("./file.typ");
  let config = default_config();
  assert!(format_text(&file_path, "#let x = (\n", &config).is_err());
  assert!(format_text_range(&file_path, "#let x = (\n", 0..5, &config).is_err());
}

#[test]
fn should_not_change_formatted_range() {
  let result = format_text_range(
    &PathBuf::from("./file.typ"),
    "#set text(size: 10pt)\n",
    0..5,
    &default_config(),
  )
  .unwrap();
  assert_eq!(result, None);
}

#[test]
fn should_not_change_range_out_of_bounds() {
  let result = format_text_range(
    &PathBuf::from("./file.typ"),
    "#set text(size: 10pt)\n",
    100..200,
    &default_config(),
  )
  .unwrap();
  assert_eq!(result, None);
}

fn default_config() -> Configuration {
  resolve_config(Default::default(), &GlobalConfiguration::default()).config
}
