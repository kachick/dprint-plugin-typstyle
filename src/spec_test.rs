use std::path::PathBuf;
use std::sync::Arc;

use super::TypstPluginHandler;
use dprint_core::configuration::{ConfigKeyMap, GlobalConfiguration};
use dprint_core::plugins::{
    FormatConfigId, NullCancellationToken, SyncFormatRequest, SyncPluginHandler,
};
use dprint_development::{ParseSpecOptions, RunSpecsOptions, ensure_no_diagnostics, run_specs};

// Spec tests verify formatting results across configuration options using dprint-development.
//
// Why spec tests instead of full E2E directories?
// 1. Fast: runs in-process during `cargo test` without building Wasm or calling the dprint CLI.
// 2. Focused: each spec file tests one option directly instead of maintaining large full-file diffs.
// 3. Idempotent: `format_twice: true` automatically verifies re-formatting stability.
//
// Real Wasm loading and CLI boundary checks are kept separately in crates/e2e as minimal smoke tests.
#[test]
fn test_specs() {
    let global_config = GlobalConfiguration::default();
    let fix_failures = std::env::var("UPDATE_SPECS")
        .map(|v| v == "1" || v == "true")
        .unwrap_or(false);

    run_specs(
        &PathBuf::from("./tests/specs"),
        &ParseSpecOptions {
            default_file_name: "default.typ",
        },
        &RunSpecsOptions {
            fix_failures,
            format_twice: true,
        },
        {
            let global_config = global_config.clone();
            Arc::new(move |path, file_text, range, spec_config| {
                let spec_config: ConfigKeyMap =
                    serde_json::from_value(spec_config.clone().into()).unwrap();
                let mut handler = TypstPluginHandler;
                let config_result = handler.resolve_config(spec_config, &global_config);
                ensure_no_diagnostics(&config_result.diagnostics);

                let token = NullCancellationToken;
                let request = SyncFormatRequest {
                    file_path: path,
                    file_bytes: file_text.as_bytes().to_vec(),
                    config_id: FormatConfigId::from_raw(1),
                    config: &config_result.config,
                    range,
                    token: &token,
                };
                let formatted = handler.format(request, |_| unreachable!())?;
                Ok(formatted.map(|bytes| String::from_utf8(bytes).unwrap()))
            })
        },
        Arc::new(move |_, _, _| panic!("Tracing is not supported.")),
    );
}
