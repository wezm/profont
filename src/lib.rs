#![no_std]

//! The [ProFont](https://web.archive.org/web/20180412214402/http://tobiasjung.name/profont/)
//! monospace programming font for use with
//! [embedded-graphics](https://github.com/jamwaffles/embedded-graphics). Font data taken from the
//! [ProFont homepage](https://web.archive.org/web/20180412214402/http://tobiasjung.name/profont/).
//!
//! # Examples
//!
//! Draw the text "Hello world" to a mock display using the 7pt Pro Font font.
//!
//! ```rust
//! use embedded_graphics::{
//!     mock_display::MockDisplay,
//!     mono_font::MonoTextStyle,
//!     pixelcolor::Rgb888,
//!     prelude::*,
//!     text::Text,
//! };
//! use profont::PROFONT_7_POINT;
//!
//! # fn main() -> Result<(), core::convert::Infallible> {
//! let mut display = MockDisplay::new();
//!
//! let text_style = MonoTextStyle::new(&PROFONT_7_POINT, Rgb888::RED);
//!
//! Text::new("Hello world", Point::new(0, 7), text_style).draw(&mut display)?;
//! # Ok(()) }
//! ```
//!
//! For a more complete example see [the example in the ssd1675
//! crate](https://github.com/wezm/ssd1675/blob/master/examples/raspberry_pi_inky_phat.rs).
//!
//! ### Glyph Coverage
//!
//! This crate provides support for [ISO/IEC 8859-1](https://en.wikipedia.org/wiki/ISO/IEC_8859-1)
//! (latin1), although do note that the font is missing a few glyphs in this range.

use embedded_graphics::{
    geometry::Size,
    image::ImageRaw,
    mono_font::{mapping::StrGlyphMapping, DecorationDimensions, MonoFont},
};

const CHARS_PER_ROW: u32 = 32;

/// Character ranges for all fonts.
///
/// This consists of two character ranges - ASCII from ' ' to '~', then ISO 8859-1 from `&nbsp;`
/// (HTML notation) to `ÿ`. Unknown characters fall back to `?`.
const GLYPH_MAPPING: StrGlyphMapping =
    StrGlyphMapping::new("\0 ~\0\u{00A0}ÿ", '?' as usize - ' ' as usize);

/// The 7 point size with a character size of 5x9 pixels.
pub const PROFONT_7_POINT: MonoFont = MonoFont {
    image: ImageRaw::new_binary(
        include_bytes!("../data/ProFont7Point.raw"),
        CHARS_PER_ROW * 5,
    ),

    glyph_mapping: &GLYPH_MAPPING,
    character_size: Size::new(5, 9),
    character_spacing: 0,
    baseline: 6,
    underline: DecorationDimensions::new(8, 1),
    strikethrough: DecorationDimensions::new(4, 1),
};

/// The 9 point size with a character size of 6x11 pixels.
pub const PROFONT_9_POINT: MonoFont = MonoFont {
    image: ImageRaw::new_binary(
        include_bytes!("../data/ProFont9Point.raw"),
        CHARS_PER_ROW * 6,
    ),

    glyph_mapping: &GLYPH_MAPPING,
    character_size: Size::new(6, 11),
    character_spacing: 0,
    baseline: 8,
    underline: DecorationDimensions::new(10, 1),
    strikethrough: DecorationDimensions::new(6, 1),
};

/// The 10 point size with a character size of 7x13 pixels.
pub const PROFONT_10_POINT: MonoFont = MonoFont {
    image: ImageRaw::new_binary(
        include_bytes!("../data/ProFont10Point.raw"),
        CHARS_PER_ROW * 7,
    ),

    glyph_mapping: &GLYPH_MAPPING,
    character_size: Size::new(7, 13),
    character_spacing: 0,
    baseline: 10,
    underline: DecorationDimensions::new(10 + 1, 1),
    strikethrough: DecorationDimensions::new(7, 1),
};

/// The 12 point size with a character size of 8x15 pixels.
pub const PROFONT_12_POINT: MonoFont = MonoFont {
    image: ImageRaw::new_binary(
        include_bytes!("../data/ProFont12Point.raw"),
        CHARS_PER_ROW * 8,
    ),

    glyph_mapping: &GLYPH_MAPPING,
    character_size: Size::new(8, 15),
    character_spacing: 0,
    baseline: 10,
    underline: DecorationDimensions::new(12, 1),
    strikethrough: DecorationDimensions::new(7, 1),
};

/// The 14 point size with a character size of 10x18 pixels.
pub const PROFONT_14_POINT: MonoFont = MonoFont {
    image: ImageRaw::new_binary(
        include_bytes!("../data/ProFont14Point.raw"),
        CHARS_PER_ROW * 10,
    ),

    glyph_mapping: &GLYPH_MAPPING,
    character_size: Size::new(10, 18),
    character_spacing: 0,
    baseline: 14,
    underline: DecorationDimensions::new(16, 1),
    strikethrough: DecorationDimensions::new(9, 1),
};

/// The 18 point size with a character size of 12x22 pixels.
pub const PROFONT_18_POINT: MonoFont = MonoFont {
    image: ImageRaw::new_binary(
        include_bytes!("../data/ProFont18Point.raw"),
        CHARS_PER_ROW * 12,
    ),

    glyph_mapping: &GLYPH_MAPPING,
    character_size: Size::new(12, 22),
    character_spacing: 0,
    baseline: 17,
    underline: DecorationDimensions::new(19, 2),
    strikethrough: DecorationDimensions::new(10, 2),
};

/// The 24 point size with a character size of 16x30 pixels.
pub const PROFONT_24_POINT: MonoFont = MonoFont {
    image: ImageRaw::new_binary(
        include_bytes!("../data/ProFont24Point.raw"),
        CHARS_PER_ROW * 16,
    ),

    glyph_mapping: &GLYPH_MAPPING,
    character_size: Size::new(16, 30),
    character_spacing: 0,
    baseline: 24,
    underline: DecorationDimensions::new(26, 2),
    strikethrough: DecorationDimensions::new(16, 2),
};
