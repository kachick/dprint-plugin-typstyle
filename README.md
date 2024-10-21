# dprint-plugin-typstyle

[![CI - Nix Status](https://github.com/kachick/dprint-plugin-typstyle/actions/workflows/nix.yml/badge.svg?branch=main)](https://github.com/kachick/dprint-plugin-typstyle/actions/workflows/nix.yml?query=branch%3Amain+)

[Typst](https://github.com/typst/typst) formatter as a [dprint](https://github.com/dprint/dprint) WASM plugin, powered by [typstyle](https://github.com/Enter-tainer/typstyle)

## Installation

```bash
dprint config add 'kachick/typstyle'
```

This plugin delegates the formatter feature to the [upstream crate](https://github.com/Enter-tainer/typstyle).

## Configuration example

```json
{
  "typst": {
    "column": 78
  },
  "plugins": [
    "https://plugins.dprint.dev/kachick/typstyle-0.2.0.wasm"
  ]
}
```
