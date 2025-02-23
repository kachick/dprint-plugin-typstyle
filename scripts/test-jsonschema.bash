#!/usr/bin/env bash

jsonschema-cli deployment/schema.json --instance <(yq --output-format json '.typst' ./tests/all/dprint.json)
