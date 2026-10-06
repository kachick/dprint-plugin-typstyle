fn main() {
    print!("{}", include_str!(concat!(env!("OUT_DIR"), "/schema.json")));
}

#[cfg(test)]
mod tests {
    // Tests here verify that the generated schema.json is a valid JSON Schema
    // and enforces the expected property constraints (defaults, enums, additionalProperties).
    //
    // We intentionally avoid depending on external E2E test fixtures (e.g. tests/*/dprint.json)
    // here. Real fixture files are naturally validated by the dprint CLI during E2E runs
    // via `dprint check`, keeping this schema generator test self-contained.
    #[test]
    fn test_generate_json_schema() {
        let schema = include_str!(concat!(env!("OUT_DIR"), "/schema.json"));
        assert!(schema.contains(r#""lineWidth":"#));
        assert!(schema.contains(r#""indentWidth":"#));
        assert!(schema.contains(r#""blankLinesUpperBound":"#));
        assert!(schema.contains(r#""reorderImportItems":"#));
        assert!(schema.contains(r#""wrapMode":"#));
        assert!(schema.contains(r#""sentence""#));
        assert!(schema.contains("https://plugins.dprint.dev/kachick/typstyle/"));
        assert!(schema.contains("/schema.json"));
        assert!(schema.contains(r#""additionalProperties": false"#));
        assert!(!schema.contains(r#""title":"#));
        assert!(!schema.contains(r#""required":"#));

        let schema_value: serde_json::Value = serde_json::from_str(&schema).unwrap();
        assert_eq!(schema_value["properties"]["lineWidth"]["default"], 80);
        assert_eq!(schema_value["properties"]["indentWidth"]["default"], 2);
        assert_eq!(
            schema_value["properties"]["blankLinesUpperBound"]["default"],
            1
        );
        assert_eq!(
            schema_value["properties"]["reorderImportItems"]["default"],
            true
        );
        assert_eq!(schema_value["properties"]["wrapMode"]["default"], "none");

        let validator = jsonschema::validator_for(&schema_value).expect("valid JSON Schema");

        // An empty config must be valid because all options are optional
        let empty_config = serde_json::json!({});
        assert!(validator.is_valid(&empty_config));

        // Valid configuration with all options specified
        let valid_all_options = serde_json::json!({
            "lineWidth": 100,
            "indentWidth": 4,
            "blankLinesUpperBound": 3,
            "reorderImportItems": false,
            "wrapMode": "fill"
        });
        assert!(validator.is_valid(&valid_all_options));

        for mode in ["none", "fill", "sentence"] {
            let valid = serde_json::json!({ "wrapMode": mode });
            assert!(validator.is_valid(&valid));
        }

        let invalid_enum = serde_json::json!({ "wrapMode": "unknown" });
        assert!(!validator.is_valid(&invalid_enum));

        let invalid_extra_key = serde_json::json!({ "unknownKey": "invalid" });
        assert!(!validator.is_valid(&invalid_extra_key));
    }
}
