use std::fs::File;
use std::io::prelude::*;

mod parser;

pub fn main() -> std::io::Result<()> {
  let lex = parser::get_lexicon().unwrap();
  let serialized = bincode::serialize(&lex).unwrap();
  println!("{:?}", serialized.len());

  let mut file = File::create("lexicon.bin")?;
  file.write_all(&serialized)?;
  Ok(())
}
