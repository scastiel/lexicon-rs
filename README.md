# Lexicon

[![Crates.io](https://img.shields.io/crates/v/lexicon)](https://crates.io/crates/lexicon)
[![Rust](https://github.com/scastiel/lexicon-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/scastiel/lexicon-rs/actions/workflows/rust.yml)

A convenient interface to get patterns from the [Lexicon](http://conwaylife.com/ref/lexicon/lex_home.htm) and use them in your implementation of [Conway’s Game of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life).

_Used by [Lifeee](https://lifeee.netlify.app), my web app implementation of Conway’s Game of Life, check it out!_

## Usage

```rust
let lexicon = lexicon::Lexicon::get();
assert_eq!(lexicon.terms.len() > 0, true);

let glider = lexicon.get_term("glider".to_string()).unwrap();
assert_eq!(glider.width, 3);
assert_eq!(glider.height, 3);
assert_eq!(glider.cells, vec![
  lexicon::Cell { x: 0, y: 0 },
  lexicon::Cell { x: 1, y: 0 },
  lexicon::Cell { x: 2, y: 0 },
  lexicon::Cell { x: 0, y: 1 },
  lexicon::Cell { x: 1, y: 2 }
]);
```

## Regenerate a bin file after an update of the text version

`cargo run` will read the file _res/lexicon.txt_, parse it and serialize the result into _res/lexicon.bin_, which will be used by the library then.
