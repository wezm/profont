# ProFont

[The ProFont monospace font][profont] for use with the [embedded-graphics] crate.

[![Build Status](https://api.cirrus-ci.com/github/wezm/profont.svg)](https://cirrus-ci.com/github/wezm/profont)
[![crates.io](https://img.shields.io/crates/v/profont.svg)](https://crates.io/crates/profont)
[![Documentation](https://docs.rs/profont/badge.svg)][crate-docs]

<img src="https://raw.githubusercontent.com/wezm/profont/master/data/IMG_2198.jpg" width="459" alt="Photo of ProFront on an ePaper display" />

`profont` is licensed under the [MIT license][MIT].

## Specimens

ProFont is available in 7 sizes:

### 7 Point

![7 Point ProFont](https://raw.githubusercontent.com/wezm/profont/master/tools/data/ProFont7Point.png)

### 9 Point

![9 Point ProFont](https://raw.githubusercontent.com/wezm/profont/master/tools/data/ProFont9Point.png)

### 10 Point

![10 Point ProFont](https://raw.githubusercontent.com/wezm/profont/master/tools/data/ProFont10Point.png)

### 12 Point

![12 Point ProFont](https://raw.githubusercontent.com/wezm/profont/master/tools/data/ProFont12Point.png)

### 14 Point

![14 Point ProFont](https://raw.githubusercontent.com/wezm/profont/master/tools/data/ProFont14Point.png)

### 18 Point

![18 Point ProFont](https://raw.githubusercontent.com/wezm/profont/master/tools/data/ProFont18Point.png)

### 24 Point

![24 Point ProFont](https://raw.githubusercontent.com/wezm/profont/master/tools/data/ProFont24Point.png)

## Examples

There are some examples that use the `embedded-graphics` simulator. Run these as follows:

    cargo run debugger
    cargo run mock-display
    cargo run hello

## Data Files

The binary of the crate is used to generate the data files from a source font. They can be regenerated with:

    bmake -C data pngs all

[embedded-graphics]: https://github.com/embedded-graphics/embedded-graphics
[profont]: https://tobiasjung.name/profont/
[MIT]: https://github.com/wezm/profont/blob/master/LICENSE
[crate-docs]: https://docs.rs/profont
