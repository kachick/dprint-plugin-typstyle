# dprint-plugin-typstyle

[![CI - Nix Status](https://github.com/kachick/dprint-plugin-typstyle/actions/workflows/nix.yml/badge.svg?branch=main)](https://github.com/kachick/dprint-plugin-typstyle/actions/workflows/nix.yml?query=branch%3Amain+)

[Typst](https://github.com/typst/typst) formatter as a [dprint](https://github.com/dprint/dprint) WASM plugin, powered by [typstyle](https://github.com/Enter-tainer/typstyle)

## Installation

```bash
dprint config add 'kachick/typstyle'
```

This plugin delegates the formatter feature to the [upstream typstyle-core crate](https://github.com/Enter-tainer/typstyle).

## Configuration example

Minimum

```json
{
  "plugins": [
    "https://plugins.dprint.dev/kachick/typstyle-0.2.6.wasm"
  ]
}
```

Customize if necessary

```json
{
  "typst": {
    "tab_spaces": 3,
    "column": 78,
    "blank_lines_upper_bound": 5
  },
  "plugins": [
    "https://plugins.dprint.dev/kachick/typstyle-0.2.6.wasm"
  ]
}
```

## Order of determines the default

1. typst section in dprint.json
1. [global config in dprint.json](https://dprint.dev/config/#global-configuration)
1. [default in typstyle-core](https://github.com/Enter-tainer/typstyle/blob/v0.12.14/crates/typstyle-core/src/config.rs#L13-L21)

## List of options

| dprint-plugin-typstyle  | dprint global config | typstyle                |
| ----------------------- | -------------------- | ----------------------- |
| column                  | lineWidth            | column(max_width)       |
| tab_spaces              | indentWidth          | tab_spaces              |
| blank_lines_upper_bound | `none`               | blank_lines_upper_bound |
