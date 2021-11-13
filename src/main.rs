use std::fs::File;
use std::io::prelude::*;

mod lex;
mod parser;

const INPUT_LEXICON_FILE: &str = "res/lexicon.txt";
const OUTPUT_LEXICON_FILE: &str = "res/lexicon.bin";

pub fn main() -> std::io::Result<()> {
  print!("Reading file {}... ", INPUT_LEXICON_FILE);
  let lex = parser::get_lexicon_from_file(INPUT_LEXICON_FILE).unwrap();
  let serialized = bincode::serialize(&lex).unwrap();
  println!("✅");

  print!("Writing result to file {}... ", OUTPUT_LEXICON_FILE);
  let mut file = File::create(OUTPUT_LEXICON_FILE)?;
  file.write_all(&serialized)?;
  println!("✅");

  Ok(())
}
