#!/usr/bin/env bash

set -euxo pipefail

env -u CARGO_BUILD_TARGET cargo run --package generate_json_schema

