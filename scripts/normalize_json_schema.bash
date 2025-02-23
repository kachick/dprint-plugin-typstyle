#!/usr/bin/env bash

version="$(yq '.package.version' Cargo.toml)"
./scripts/generate_json_schema.rs |
	yq --output-format json "del(.required, .title) |
		.\"\$id\" = \"https://plugins.dprint.dev/kachick/typstyle/${version}/schema.json\" |
		.additionalProperties = false" >./deployment/schema.json
