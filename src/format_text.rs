use std::ops::Range;
use std::path::Path;

use typst_syntax::Source;
use typstyle_core::Typstyle;

use crate::configuration::Configuration;

/// Error that occurred while formatting Typst text.
#[derive(Debug, thiserror::Error)]
pub enum FormatError {
  /// The text could not be formatted by typstyle.
  #[error(transparent)]
  Typstyle(#[from] typstyle_core::Error),
}

pub fn format_text(_file_path: &Path, input_text: &str, config: &Configuration) -> Result<Option<String>, FormatError> {
  let formatted = Typstyle::new(build_config(config)).format_text(input_text).render()?;
  Ok(if formatted == input_text { None } else { Some(formatted) })
}

/// Formats only the part of the text within the provided byte range.
///
/// typstyle widens the range to the syntax nodes it touches and the text outside of those is left
/// as it was.
pub fn format_text_range(
  _file_path: &Path,
  input_text: &str,
  range: Range<usize>,
  config: &Configuration,
) -> Result<Option<String>, FormatError> {
  let range = clamp_to_char_boundaries(input_text, range);
  let result = Typstyle::new(build_config(config)).format_source_range(Source::detached(input_text), range)?;

  let source_range = result.source_range;
  if source_range.start > source_range.end || source_range.end > input_text.len() {
    return Ok(None);
  }

  let mut formatted = String::with_capacity(input_text.len() - source_range.len() + result.content.len());
  formatted.push_str(&input_text[..source_range.start]);
  formatted.push_str(&result.content);
  formatted.push_str(&input_text[source_range.end..]);
  Ok(if formatted == input_text { None } else { Some(formatted) })
}

fn build_config(config: &Configuration) -> typstyle_core::Config {
  typstyle_core::Config {
    tab_spaces: config.indent_width as usize,
    max_width: config.line_width as usize,
    blank_lines_upper_bound: config.blank_lines_upper_bound as usize,
    reorder_import_items: config.reorder_import_items,
    wrap_mode: config.wrap_mode.into(),
    ..Default::default()
  }
}

/// Clamps the range to the text and widens it to the nearest char boundaries.
fn clamp_to_char_boundaries(text: &str, range: Range<usize>) -> Range<usize> {
  let mut end = range.end.min(text.len());
  let mut start = range.start.min(end);
  while !text.is_char_boundary(start) {
    start -= 1;
  }
  while !text.is_char_boundary(end) {
    end += 1;
  }
  start..end
}
