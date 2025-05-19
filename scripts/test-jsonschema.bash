#!/usr/bin/env bash

set -euxo pipefail

export SCHEMA_PATH="${SCHEMA_PATH:-deployment/schema.json}"

jsonschema-cli "$SCHEMA_PATH" --instance <(yq --output-format json '.typst' ./tests/all/dprint.json)
