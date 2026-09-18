use std::fmt;
use std::str::FromStr;

use schemars::{JsonSchema, schema_for};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, JsonSchema, Default)]
#[serde(rename_all = "kebab-case")]
pub enum WrapMode {
    #[default]
    None,
    Fill,
    Sentence,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseWrapModeError(String);

impl fmt::Display for ParseWrapModeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Invalid wrapMode: '{}'. Expected 'none', 'fill', or 'sentence'.",
            self.0
        )
    }
}

impl std::error::Error for ParseWrapModeError {}

impl FromStr for WrapMode {
    type Err = ParseWrapModeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(WrapMode::None),
            "fill" => Ok(WrapMode::Fill),
            "sentence" => Ok(WrapMode::Sentence),
            _ => Err(ParseWrapModeError(s.to_string())),
        }
    }
}

impl From<WrapMode> for typstyle_core::WrapMode {
    fn from(mode: WrapMode) -> Self {
        match mode {
            WrapMode::None => typstyle_core::WrapMode::None,
            WrapMode::Fill => typstyle_core::WrapMode::Fill,
            WrapMode::Sentence => typstyle_core::WrapMode::Sentence,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
// Adjust names with dprint global configuration
//
// List of upstream:
//   dprint: https://github.com/dprint/dprint/blob/0.57.4/crates/core/src/configuration.rs#L257-L289
//   typstyle: https://github.com/typstyle-rs/typstyle/blob/v0.15.1/crates/typstyle-core/src/config.rs#L5-L18
// TODO: Remove required from all options in json schema. See https://github.com/GREsau/schemars/issues/344
//
// Don't add "collapse_markup_spaces" to config schema until upstream open it in the CLI
// See https://github.com/typstyle-rs/typstyle/pull/302#discussion_r2104164153 for detail
pub struct Configuration {
    // column/max_width in typstyle-core
    pub line_width: u32,

    // tab_spaces in typstyle-core
    pub indent_width: u8,

    // None in dprint
    pub blank_lines_upper_bound: u32,

    // Sort import items in a single import statement
    pub reorder_import_items: bool,

    // Text wrapping mode for markup
    pub wrap_mode: WrapMode,
}

impl From<&Configuration> for typstyle_core::Config {
    fn from(config: &Configuration) -> Self {
        typstyle_core::Config {
            tab_spaces: config.indent_width as usize,
            max_width: config.line_width as usize,
            blank_lines_upper_bound: config.blank_lines_upper_bound as usize,
            reorder_import_items: config.reorder_import_items,
            wrap_mode: config.wrap_mode.into(),
            ..Default::default()
        }
    }
}

#[must_use]
pub fn generate_json_schema() -> String {
    let mut schema = serde_json::to_value(schema_for!(Configuration)).unwrap();
    let version = env!("CARGO_PKG_VERSION");
    if let Some(obj) = schema.as_object_mut() {
        obj.remove("title");
        obj.remove("required");
        obj.insert(
            "$id".to_string(),
            serde_json::Value::String(format!(
                "https://plugins.dprint.dev/kachick/typstyle/{version}/schema.json"
            )),
        );
        obj.insert(
            "additionalProperties".to_string(),
            serde_json::Value::Bool(false),
        );
    }
    serde_json::to_string_pretty(&schema).unwrap()
}

#[test]
fn test_generate_json_schema() {
    let schema = generate_json_schema();
    assert!(schema.contains(r#""lineWidth":"#));
    assert!(schema.contains(r#""indentWidth":"#));
    assert!(schema.contains(r#""blankLinesUpperBound":"#));
    assert!(schema.contains(r#""reorderImportItems":"#));
    assert!(schema.contains(r#""wrapMode":"#));
    assert!(schema.contains(r#""sentence""#));
    assert!(schema.contains(r#""$id": "https://plugins.dprint.dev/kachick/typstyle/"#));
    assert!(schema.contains(r#""additionalProperties": false"#));
    assert!(!schema.contains(r#""title":"#));
    assert!(!schema.contains(r#""required":"#));
}

#[test]
fn test_wrap_mode_from_str() {
    assert_eq!("none".parse::<WrapMode>(), Ok(WrapMode::None));
    assert_eq!("fill".parse::<WrapMode>(), Ok(WrapMode::Fill));
    assert_eq!("sentence".parse::<WrapMode>(), Ok(WrapMode::Sentence));
    let err = "invalid".parse::<WrapMode>().unwrap_err();
    assert_eq!(
        err.to_string(),
        "Invalid wrapMode: 'invalid'. Expected 'none', 'fill', or 'sentence'."
    );
}
