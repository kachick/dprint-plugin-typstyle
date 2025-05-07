use schemars::{JsonSchema, schema_for};
use serde::Serialize;

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
#[derive(JsonSchema)]
// Adjust names with dprint global configuration
//
// List of upstream:
//   dprint: https://github.com/dprint/dprint/blob/0.49.0/crates/core/src/configuration.rs#L257-L278
//   typestyle: https://github.com/Enter-tainer/typstyle/blob/v0.12.14/crates/typstyle-core/src/config.rs#L4-L11
// TODO: Remove required from all options in json schema. See https://github.com/GREsau/schemars/issues/344
pub struct Configuration {
    // column/max_width in typstyle-core
    pub line_width: u32,

    // tab_spaces in typstyle-core
    pub indent_width: u8,

    // None in dprint
    pub blank_lines_upper_bound: u32,
    // Expose this field after updating to 0.13.4 or later, "Experimental" flag was removed in that version
    // pub reorder_import_items: bool,
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
}
