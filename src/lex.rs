use serde::{Deserialize, Serialize};
use std::fmt::Debug;

/// Struct representing a cell.
#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Cell {
  /// x coordinate (horizontal position).
  pub x: i32,
  /// y coordinate (vertical position).
  pub y: i32,
}

/// Struct representing a term from a lexicon, i.e. a pattern.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Term {
  /// Name of the term.
  pub name: String,
  /// Long description of the term. Links to other terms are enclosed in
  /// curly braces, e.g. "See also {glider}."
  pub description: String,
  /// Tags associated to the term, e.g. "p12", "c/2 orthogonally".
  pub tags: Vec<String>,
  /// Initial alive cells of the term.
  pub cells: Vec<Cell>,
  /// Initial width of the term’s pattern.
  pub width: usize,
  /// Initial height of the term’s pattern.
  pub height: usize,
}

/// Struct representing the content of a lexicon.
#[derive(Debug, Serialize, Deserialize)]
pub struct Lexicon {
  /// All the terms extracted from the lexicon.
  ///
  /// ```
  /// let lexicon = lexicon::Lexicon::get();
  /// assert_eq!(lexicon.terms.len() > 10, true);
  /// ```
  pub terms: Vec<Term>,
}

impl Lexicon {
  /// Look for a term by its name.
  ///
  /// # Examples
  ///
  /// ## With known term name
  ///
  /// ```
  /// let lexicon = lexicon::Lexicon::get();
  /// let glider = lexicon.get_term("glider".to_string()).unwrap();
  /// assert_eq!(glider.name, "glider".to_string());
  /// ```
  ///
  /// ## With unknown term name
  ///
  /// ```
  /// let lexicon = lexicon::Lexicon::get();
  /// let glider = lexicon.get_term("unknwon pattern".to_string());
  /// assert_eq!(glider.is_none(), true);
  /// ```
  pub fn get_term(&self, name: String) -> Option<&Term> {
    self.terms.iter().find(|term| term.name == name)
  }

  /// Returns the default official lexicon.
  ///
  /// # Examples
  ///
  /// ```
  /// let lexicon = lexicon::Lexicon::get();
  /// ```
  ///
  /// # Panics
  ///
  /// If for some reason the content of the default binary file containing
  /// the lexicon can’t be parsed.
  pub fn get() -> Lexicon {
    let serialized = include_bytes!("../res/lexicon.bin");
    let lexicon: Lexicon = bincode::deserialize(serialized).unwrap();
    lexicon
  }
}
