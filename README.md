# cargo_aoc_template

This is a basic rust template for solving AdventOfCode (aoc) tasks.
It is meant to be used with [cargo generate](https://cargo-generate.github.io/cargo-generate/index.html).

## Basic Usage

```cmd
cargo generate -f https://github.com/jgpr-code/cargo_aoc_template.git
```

*Note*: The -f flag disables automatic renaming of the project name to
kebab-case.

## Setup as Favorite

``cargo-generate`` has support for setting up favorite templates locally.
This is how you can do it:

Create a ``cargo-generate.toml`` in your ``$HOME\.cargo`` folder with the
following contents:

```toml
[favorites.aoc]
description = "Rust template for Advent of Code"
git = "https://github.com/jgpr-code/cargo_aoc_template"
```

Afterwards usage simply becomes:

```cmd
cargo generate -f aoc
```

Way easier to remember. Nice!
