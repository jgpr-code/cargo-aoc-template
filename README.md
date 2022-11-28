# cargo_aoc_template

This is a basic rust template for solving AdventOfCode (aoc) tasks.
It is meant to be used with [cargo generate](https://cargo-generate.github.io/cargo-generate/index.html).

## Basic usage

```cmd
cargo generate -f https://github.com/jgpr-code/cargo_aoc_template.git
```

*Note*: The -f flag disables automatic renaming of the project name to
kebab-case.

## Setup as favorite

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

## How to use this template

1. Populate the contents of ``input.txt`` and ``test.txt`` with your input
   from the website (``test.txt`` is optional for examples directly in the text)
2. Write an input parser in the function ``parse_buffer`` returning
   a customized ``Input`` struct
3. Work with the ``Input`` struct on the solutions for ``part_one`` and
   ``part_two``
4. Test your solution with the already present tests😀
