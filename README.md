# cargo_aoc_template

🎄⭐🎄⭐🎄 Advent of Code 🎄⭐🎄⭐🎄

This is my rust template for solving AdventOfCode (aoc) tasks.
It is meant to be used with [cargo generate](https://cargo-generate.github.io/cargo-generate/index.html).

## Basic usage

```cmd
cargo generate https://github.com/jgpr-code/cargo_aoc_template.git
```

## Setup as favorite

`cargo-generate` has support for setting up favorite templates locally.
This is how you can do it:

Create a `cargo-generate.toml` in your `$HOME\.cargo` folder with the
following contents:

```toml
[favorites.aoc]
description = "Rust template for Advent of Code"
git = "https://github.com/jgpr-code/cargo_aoc_template"
```

Afterwards usage simply becomes:

```cmd
cargo generate aoc
```

Way easier to remember. Nice👍

## Using the template

This template uses a virtual rust workspace, containing a `common` library crate and `dayXY` binary crates.

The `common` library initially offers the `Answer` struct used in the days and a more
ergonomic `regx` macro for Rusts regex library.
Additionally, it has some utility code to support the test code.

### Prerequisites

The templates tests require a nightly installation of Rust.

```cmd
rustup toolchain install nightly
```

### Solving Days

1. Open the workbench just for your `dayXY` at hand.
2. Populate the contents of `input` and `test` with your input from the website
   or with [aoc-cli](https://github.com/scarvalhojr/aoc-cli) (`aoc d -Io`)
3. Write an input parser in the function `parse_input` returning
   a customized `Input` struct (feel free to rename it)
4. Write the solutions in `solve_one` and `solve_two`
5. Test your solutions with the updated tests by running `cargo test <one|two> [-r]`
6. ⭐⭐
