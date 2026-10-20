use std::fmt;
use std::str::FromStr;

use serde::Deserialize;
use serde::Serialize;

// Adjust names with dprint global configuration
//
// List of upstream:
//   dprint: https://github.com/dprint/dprint/blob/0.58.0/crates/core/src/configuration.rs#L257-L289
//   typstyle: https://github.com/typstyle-rs/typstyle/blob/v0.15.1/crates/typstyle-core/src/config.rs#L5-L18
//
// Don't add "collapse_markup_spaces" to config schema until upstream open it in the CLI.
// At least as of typstyle 0.15.1, it is still not open in the CLI.
// See https://github.com/typstyle-rs/typstyle/pull/302#discussion_r2104164153 for detail
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Configuration {
  // column/max_width in typstyle-core
  pub line_width: u32,

  // tab_spaces in typstyle-core
  pub indent_width: u8,

  // None in dprint
  pub blank_lines_upper_bound: u32,

  // Sort import items in a single import statement
  pub reorder_import_items: bool,

  // Text wrapping mode for markup
  pub wrap_mode: WrapMode,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum WrapMode {
  #[default]
  None,
  Fill,
  Sentence,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseWrapModeError(String);

impl fmt::Display for ParseWrapModeError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "Invalid wrapMode: '{}'. Expected 'none', 'fill', or 'sentence'.",
      self.0
    )
  }
}

impl std::error::Error for ParseWrapModeError {}

impl FromStr for WrapMode {
  type Err = ParseWrapModeError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "none" => Ok(WrapMode::None),
      "fill" => Ok(WrapMode::Fill),
      "sentence" => Ok(WrapMode::Sentence),
      _ => Err(ParseWrapModeError(s.to_string())),
    }
  }
}

impl From<WrapMode> for typstyle_core::WrapMode {
  fn from(mode: WrapMode) -> Self {
    match mode {
      WrapMode::None => typstyle_core::WrapMode::None,
      WrapMode::Fill => typstyle_core::WrapMode::Fill,
      WrapMode::Sentence => typstyle_core::WrapMode::Sentence,
    }
  }
}
