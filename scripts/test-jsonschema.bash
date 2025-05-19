#!/usr/bin/env bash

set -euxo pipefail

export SCHEMA_PATH="${SCHEMA_PATH:-/dev/stdin}"

jsonschema-cli "$SCHEMA_PATH" --instance <(yq --output-format json '.typst' ./tests/all/dprint.json)
grep --quiet --fixed-strings "$VERSION" "$SCHEMA_PATH"
