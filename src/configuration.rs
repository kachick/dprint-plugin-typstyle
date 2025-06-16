use schemars::{JsonSchema, schema_for};
use serde::Serialize;

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(JsonSchema)]
// Adjust names with dprint global configuration
//
// List of upstream:
//   dprint: https://github.com/dprint/dprint/blob/0.49.1/crates/core/src/configuration.rs#L257-L278
//   typestyle: https://github.com/Enter-tainer/typstyle/blob/v0.13.5/crates/typstyle-core/src/config.rs#L4-L16
// TODO: Remove required from all options in json schema. See https://github.com/GREsau/schemars/issues/344
//
// Don't add "collapse_markup_spaces" to config schema until upstream open it in the CLI
// See https://github.com/Enter-tainer/typstyle/pull/302#discussion_r2104164153 for detail
pub struct Configuration {
    // column/max_width in typstyle-core
    pub line_width: u32,

    // tab_spaces in typstyle-core
    pub indent_width: u8,

    // None in dprint
    pub blank_lines_upper_bound: u32,

    // Sort import items in a single import statement
    pub reorder_import_items: bool,

    // Wrap texts in the markup
    pub wrap_text: bool,
}

pub fn generate_json_schema() -> String {
    let schema = schema_for!(Configuration);
    serde_json::to_string_pretty(&schema).unwrap()
}

#[test]
fn test_generate_json_schema() {
    let schema = generate_json_schema();
    assert!(schema.contains(r#""lineWidth":"#));
    assert!(schema.contains(r#""indentWidth":"#));
    assert!(schema.contains(r#""blankLinesUpperBound":"#));
    assert!(schema.contains(r#""reorderImportItems":"#));
    assert!(schema.contains(r#""wrapText":"#));
}
