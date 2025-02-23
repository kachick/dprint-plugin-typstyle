#!/usr/bin/env bash

./scripts/generate_json_schema.rs |
	jq 'del(.required, .title) |
		."$id" = "https://plugins.dprint.dev/kachick/typstyle/0.3.0/schema.json" |
		.additionalProperties = false
		' >./deployment/schema.json
