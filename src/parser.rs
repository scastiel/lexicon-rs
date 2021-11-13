use crate::lex::*;
use regex::Regex;
use std::error::Error;
use std::fmt;
use std::fmt::Debug;
use std::fs::read_to_string;

#[derive(Debug)]
pub struct ParseError(String);

impl fmt::Display for ParseError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Parse error: {}", self.0)
  }
}

impl Error for ParseError {
  fn description(&self) -> &str {
    self.0.as_str()
  }
}

fn read_term_name(line: String) -> Option<(String, Vec<String>, String)> {
  let re = Regex::new(r"^:(?P<name>[^:]+): (\((?P<tags>[^)]+)\))?(?P<desc>.*)$").unwrap();
  re.captures(line.as_str()).map(|cap| {
    let name = cap
      .name("name")
      .map(|name| name.as_str().to_string())
      .unwrap();
    let tags = cap
      .name("tags")
      .map(|name| {
        name
          .as_str()
          .split(", ")
          .map(|s| s.to_string())
          .collect::<Vec<String>>()
      })
      .unwrap_or(vec![]);
    let desc = cap
      .name("desc")
      .map(|desc| desc.as_str().trim().to_string())
      .unwrap();
    (name, tags, desc)
  })
}

fn read_cells_line(line: String, y: i32) -> Vec<Cell> {
  let line = line.strip_prefix("\t").unwrap();
  let mut cells = vec![];
  for (x, c) in line.char_indices() {
    if c == '*' {
      cells.push(Cell { x: x as i32, y });
    }
  }
  cells
}

fn parse_lexicon(lexicon_txt: String) -> Result<Lexicon, ParseError> {
  let mut lines = lexicon_txt.lines();

  loop {
    match lines.next() {
      Some(line) => {
        if line.to_string().starts_with("----") {
          break;
        }
      }
      _ => return Err(ParseError("Unexpected end of input".to_string())),
    }
  }

  let mut lexicon = Lexicon { terms: vec![] };

  loop {
    match lines.next() {
      Some(line) => {
        let line = line.to_string();
        if line.starts_with("----") {
          break;
        }
        if line.starts_with(":") {
          if let Some((name, tags, desc)) = read_term_name(line) {
            let mut current_term = Term {
              name,
              description: desc,
              tags,
              cells: vec![],
              width: 0,
              height: 0,
            };

            let mut cells_line = 0;
            let mut pattern_started = false;
            let mut pattern_ended = false;
            loop {
              match lines.next() {
                Some(line) => {
                  let line = line.to_string();
                  if line.is_empty() {
                    break;
                  }
                  if line.starts_with("   ") {
                    if pattern_started {
                      pattern_ended = true;
                    }
                    current_term.description += " ";
                    current_term.description += line.trim();
                  }
                  if !pattern_ended && line.starts_with("\t") {
                    pattern_started = true;
                    current_term.width = line.len() - 1;
                    let mut cells = read_cells_line(line, cells_line);
                    current_term.cells.append(&mut cells);
                    cells_line += 1;
                  }
                }
                _ => return Err(ParseError("Unexpected end of input".to_string())),
              }
            }

            current_term.height = cells_line as usize;
            lexicon.terms.push(current_term);
          } else {
            return Err(ParseError("Can't parse term name.".to_string()));
          }
        }
      }
      _ => return Err(ParseError("Unexpected end of input".to_string())),
    }
  }

  Ok(lexicon)
}

/// Parses a text lexicon file and returns the resulting Lexicon struct.
///
/// # Examples
///
/// ```
/// let lexicon = lexicon::get_lexicon_from_file("res/lexicon.txt");
/// assert_eq!(lexicon.is_ok(), true);
/// ```
pub fn get_lexicon_from_file(filename: &str) -> Result<Lexicon, Box<dyn Error>> {
  let lexicon_txt = read_to_string(filename)?;
  let lexicon = parse_lexicon(lexicon_txt)?;
  Ok(lexicon)
}

#[cfg(test)]
mod tests {
  use super::*;

  const LEXICON_FILE: &str = "res/lexicon.txt";

  fn get_test_lexicon() -> Lexicon {
    let lexicon = get_lexicon_from_file(LEXICON_FILE);
    assert_eq!(lexicon.is_ok(), true);
    lexicon.unwrap()
  }

  #[test]
  fn demonoid() {
    let lexicon = get_test_lexicon();
    let demonoid = lexicon.get_term("0hd Demonoid".to_string());
    assert_eq!(demonoid.is_some(), true);
    let demonoid = demonoid.unwrap();
    assert_eq!(demonoid.name, "0hd Demonoid".to_string());
    assert_eq!(demonoid.description, "See {Demonoid}.".to_string());
    assert_eq!(demonoid.cells.len(), 0);
    assert_eq!(demonoid.height, 0);
    assert_eq!(demonoid.width, 0);
    assert_eq!(demonoid.tags, Vec::<String>::new());
  }

  #[test]
  fn gosper_glider_gun() {
    let lexicon = get_test_lexicon();
    let gun = lexicon.get_term("Gosper glider gun".to_string());
    assert_eq!(gun.is_some(), true);
    let gun = gun.unwrap();
    assert_eq!(gun.name, "Gosper glider gun".to_string());
    assert_eq!(gun.cells.len(), 36);
    assert_eq!(gun.height, 9);
    assert_eq!(gun.width, 36);
    assert_eq!(gun.tags, Vec::<String>::new());
  }

  #[test]
  fn mickey_mouse() {
    let lexicon = get_test_lexicon();
    let mickey = lexicon.get_term("Mickey Mouse".to_string());
    assert_eq!(mickey.is_some(), true);
    let mickey = mickey.unwrap();
    assert_eq!(
      mickey.description,
      "The following {still life}, named by Mark Niemiec:".to_string()
    );
    assert_eq!(mickey.tags, vec!["p1".to_string()]);
  }

  #[test]
  fn pufferfish() {
    let lexicon = get_test_lexicon();
    let pufferfish = lexicon.get_term("pufferfish".to_string());
    assert_eq!(pufferfish.is_some(), true);
    let pufferfish = pufferfish.unwrap();
    assert_eq!(pufferfish.tags, vec!["c/2".to_string(), "p12".to_string()]);
    assert_eq!(pufferfish.description, "A puffer discovered by Richard Schank in November 2014, from a symmetric soup search using an early version of {apgsearch}.  It consists of a pair of {B-heptomino}es stabilised by a backend that leaves only pairs of blocks behind.  It is simple enough to be easily synthesized with gliders. See {soup} for a random initial pattern, generated by {apgsearch} and recorded in {Catagolue}, that produces a pufferfish.".to_string());
  }

  #[test]
  fn garden_of_eden() {
    let lexicon = get_test_lexicon();
    let garden = lexicon.get_term("Garden of Eden".to_string()).unwrap();
    assert_eq!(garden.height, 12);
  }
}
