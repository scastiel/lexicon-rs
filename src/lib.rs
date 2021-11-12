use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Cell {
  pub x: i32,
  pub y: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Term {
  pub name: String,
  pub description: String,
  pub tags: Vec<String>,
  pub cells: Vec<Cell>,
  pub width: usize,
  pub height: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Lexicon {
  pub terms: Vec<Term>,
}

impl Lexicon {
  pub fn get_term(&self, name: String) -> Option<&Term> {
    self.terms.iter().find(|term| term.name == name)
  }
}
