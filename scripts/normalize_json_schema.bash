#!/usr/bin/env bash

set -euxo pipefail

export SCHEMA_PATH="${SCHEMA_PATH:-deployment/schema.json}"

version="$(yq '.package.version' Cargo.toml)"
cargo run --package generate_json_schema |
	yq --output-format json "del(.required, .title) |
		.\"\$id\" = \"https://plugins.dprint.dev/kachick/typstyle/${version}/schema.json\" |
		.additionalProperties = false" >"$SCHEMA_PATH"
