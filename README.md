# dprint-plugin-typstyle

[![CI - Nix Status](https://github.com/kachick/dprint-plugin-typstyle/actions/workflows/nix.yml/badge.svg?branch=main)](https://github.com/kachick/dprint-plugin-typstyle/actions/workflows/nix.yml?query=branch%3Amain+)

[Typst](https://github.com/typst/typst) formatter as a [dprint](https://github.com/dprint/dprint) WASM plugin, powered by [typstyle](https://github.com/Enter-tainer/typstyle)

## Installation

```bash
dprint config add 'kachick/typstyle'
```

This plugin delegates the formatter feature to the upstream typstyle-core [crate](https://crates.io/crates/typstyle).

## Configuration example

Minimum

```json
{
  "plugins": [
    "https://plugins.dprint.dev/kachick/typstyle-0.3.4.wasm"
  ]
}
```

Customize if necessary

```json
{
  "typst": {
    "indentWidth": 3,
    "lineWidth": 78,
    "blankLinesUpperBound": 5
  },
  "plugins": [
    "https://plugins.dprint.dev/kachick/typstyle-0.3.4.wasm"
  ]
}
```

## Order of determines the values

1. typst section in dprint.json
1. [global config in dprint.json](https://dprint.dev/config/#global-configuration)
1. [default values in typstyle-core](https://github.com/Enter-tainer/typstyle/blob/v0.13.5/crates/typstyle-core/src/config.rs#L18-L28)

## Relationships of the option names

| dprint-plugin-typstyle | dprint global config | typstyle-core           | typstyle CLI                      |
| ---------------------- | -------------------- | ----------------------- | --------------------------------- |
| lineWidth              | lineWidth            | max_width               | column                            |
| indentWidth            | indentWidth          | tab_spaces              | tab-width                         |
| blankLinesUpperBound   | `none`               | blank_lines_upper_bound | `none`                            |
| reorderImportItems     | `none`               | reorder_import_items    | no-reorder-import-items # disable |
| wrapText               | `none`               | wrap_text               | wrap-text                         |

## Versioning

Versions are updated based on changes in dprint and typestyle-core, and do not correspond to upstream version numbers.
