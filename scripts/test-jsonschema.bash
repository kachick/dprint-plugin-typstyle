#!/usr/bin/env bash

jsonschema-cli deployment/schema.json --instance <(jq '.typst' ./tests/all/dprint.json)
