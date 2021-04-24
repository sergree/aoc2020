# Advent of Code 2020: Мade with Rust 

When I was solving [puzzles](https://adventofcode.com/2020), my goal was to practice writing *idiomatic Rust*. My solutions do not claim to be *the fastest* or *fully production ready*. Consider this as the 2020 version of [BurntSushi's 2018 Advent of Code solutions][BurntSushi]. I tried to maintain it's style by adding my own features.

- 🧘 *Panicless* idiomatic Rust code
- ☂️ [DRY](https://en.wikipedia.org/wiki/Don%27t_repeat_yourself): shared codebase between the two parts
- 1️⃣ Single `main.rs` per day
- 🎿 *Acceptable* speed
- 🤷 [anyhow](https://github.com/dtolnay/anyhow) error handling
- 🌞 Doesn't need Nightly

The solution runs the same way as [BurntSushi's][BurntSushi]: `cd` into it's directory and invoke the program with Cargo:

```
$ cd aoc01
$ cargo run --release < input/input.txt
```

## MSRV

The minimum supported Rust version is **1.51** due to [Const Generics](https://github.com/rust-lang/rust/pull/79135).

## My Favorite Alternatives

- The fastest AoC 2020 Rust solutions: https://github.com/timvisee/advent-of-code-2020
- Production grade AoC 2020 Rust solutions: https://github.com/coriolinus/adventofcode-2020

Check them out too!

[BurntSushi]: https://github.com/BurntSushi/advent-of-code
