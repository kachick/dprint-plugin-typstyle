fn main() {
    println!(
        "{}",
        dprint_plugin_typstyle::configuration::generate_json_schema()
    );
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_generate_json_schema() {
        let schema = dprint_plugin_typstyle::configuration::generate_json_schema();
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
        let validator = jsonschema::validator_for(&schema_value).expect("valid JSON Schema");

        let fixture: serde_json::Value =
            serde_json::from_str(include_str!("../../../tests/all/dprint.json")).unwrap();
        assert!(validator.is_valid(&fixture["typst"]));

        for mode in ["none", "fill", "sentence"] {
            let valid = serde_json::json!({ "wrapMode": mode });
            assert!(validator.is_valid(&valid));
        }

        let invalid = serde_json::json!({ "wrapMode": "unknown" });
        assert!(!validator.is_valid(&invalid));
    }
}
