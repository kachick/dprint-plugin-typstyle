# dprint-plugin-typstyle

[![npm version](https://img.shields.io/npm/v/@dprint/typstyle.svg)](https://www.npmjs.com/package/@dprint/typstyle) [![CI](https://github.com/dprint/dprint-plugin-typstyle/workflows/CI/badge.svg)](https://github.com/dprint/dprint-plugin-typstyle/actions?query=workflow%3ACI)

Adapter for [typstyle](https://github.com/typstyle-rs/typstyle) for use as a formatting plugin in [dprint](https://github.com/dprint/dprint). It formats [Typst](https://github.com/typst/typst) files (`.typ`).

Originally created by [Kenichi Kamiya](https://github.com/kachick).

## Install

[Install](https://dprint.dev/install/) and [setup](https://dprint.dev/setup/) dprint.

Then in your project's directory with a dprint.json file, run:

```shellsession
dprint add typstyle
```

Note: You do not need typstyle installed globally as dprint will run typstyle-core from the .wasm file in a sandboxed environment.

## Configuration

Works with no configuration by default. To add configuration, specify a `"typst"` key in your dprint.json:

```jsonc
{
  "typst": {
    "wrapMode": "fill", // "none" by default
  },
  "plugins": [
    // ...etc...
  ],
}
```

### Configuration precedence

1. typst section in dprint.json
1. [global config in dprint.json](https://dprint.dev/config/#global-configuration)
1. [default values in typstyle-core](https://github.com/typstyle-rs/typstyle/blob/v0.15.1/crates/typstyle-core/src/config.rs#L36-L43)

### Option name mappings

| dprint-plugin-typstyle | dprint global config | typstyle-core           | typstyle CLI                         |
| ---------------------- | -------------------- | ----------------------- | ------------------------------------ |
| lineWidth              | lineWidth            | max_width               | line-width                           |
| indentWidth            | indentWidth          | tab_spaces              | indent-width                         |
| blankLinesUpperBound   | `none`               | blank_lines_upper_bound | `none`                               |
| reorderImportItems     | `none`               | reorder_import_items    | no-reorder-import-items # disable    |
| wrapMode               | `none`               | wrap_mode               | wrap-text (`none`/`fill`/`sentence`) |

## JS Formatting API

- [JS Formatter](https://github.com/dprint/js-formatter) - Browser/Deno and Node
- [npm package](https://www.npmjs.com/package/@dprint/typstyle)

## Versioning

This repo automatically upgrades to the latest version of typstyle once a day. You can check which version of typstyle is being used by looking at the `typstyle-core` entry in the Cargo.toml file in this repo:

https://github.com/dprint/dprint-plugin-typstyle/blob/main/Cargo.toml

At the moment, the version of this plugin does not reflect the version of typstyle.

## License

This project is licensed under the Apache License 2.0 - see the [LICENSE](LICENSE) and [NOTICE](NOTICE) files for details.
