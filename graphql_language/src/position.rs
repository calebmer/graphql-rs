/// The position of a single character in a source document.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Position {
  /// The 0-indexed position of the character. If the source document was an
  /// array of characters, this would be the index position of that character.
  pub index: usize,
  /// The 1-indexed line on which this character lies. This number is helpful
  /// when it comes to text editors. Used in conjunction with `column`.
  pub line: usize,
  /// The 1-indexed column on which this character lies. This number is helpful
  /// when it comes to text editors. Used in conjunction with `line`.
  pub column: usize,
}

/// Contains a range of UTF-8 character offsets and token references that
/// identify the region of the source from which the AST derived.
#[derive(Clone, PartialEq, Debug)]
pub struct Location {
  /// The character position at which this Node begins.
  pub start: Position,
  /// The character position at which this Node ends.
  pub end: Position,
}

impl Location {
  /// Creates a new location between these two bounds.
  pub fn new(start: Position, end: Position) -> Self {
    Location {
      start: start,
      end: end,
    }
  }
}
