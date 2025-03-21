#!/usr/bin/env bash

set -euxo pipefail

jsonschema-cli deployment/schema.json --instance <(yq --output-format json '.typst' ./tests/all/dprint.json)
