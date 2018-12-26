//! The [ProFont](https://web.archive.org/web/20180412214402/http://tobiasjung.name/profont/)
//! monospace programming font for use with
//! [embedded-graphics](https://github.com/jamwaffles/embedded-graphics). Font data taken from the
//! [ProFont homepage](https://web.archive.org/web/20180412214402/http://tobiasjung.name/profont/).
//!
//! ### Synopsis
//!
//! Assuming `display` is something that implements the [Drawing
//! trait](https://docs.rs/embedded-graphics/0.4.4/embedded_graphics/trait.Drawing.html)
//!
//! ```ignore
//! display.draw(
//!     ProFont24Point::render_str("Hello World")
//!         .with_stroke(Some(Color::Red))
//!         .with_fill(Some(Color::White))
//!         .translate(Coord::new(10, 10))
//!         .into_iter(),
//! );
//! ```
//!
//! For a more complete example see [the example in the ssd1675
//! crate](https://github.com/wezm/ssd1675/blob/master/examples/raspberry_pi_inky_phat.rs).
//!
//! ### Glyph Coverage
//!
//! This crate provides support for [ISO/IEC 8859-1](https://en.wikipedia.org/wiki/ISO/IEC_8859-1)
//! (latin1), although do note that the font is missing a few glyphs in this range.

extern crate embedded_graphics;

use embedded_graphics::fonts::font_builder::{FontBuilder, FontBuilderConf};

const CHARS_PER_ROW: u32 = 32;

fn char_offset_impl(c: char) -> u32 {
    let fallback = '?' as u32 - ' ' as u32;
    if c < ' ' {
        return fallback;
    }
    if c <= '~' {
        return c as u32 - ' ' as u32;
    }
    if c < '\u{00A0}' || c > 'ÿ' {
        return fallback;
    }
    c as u32 - ' ' as u32 - 33
}

#[derive(Debug, Copy, Clone)]
pub enum ProFont7PointConf {}
impl FontBuilderConf for ProFont7PointConf {
    const FONT_IMAGE: &'static [u8] = include_bytes!("../data/ProFont7Point.raw");
    const CHAR_HEIGHT: u32 = 9;
    const CHAR_WIDTH: u32 = 5;
    const FONT_IMAGE_WIDTH: u32 = Self::CHAR_WIDTH * CHARS_PER_ROW;
    fn char_offset(c: char) -> u32 {
        char_offset_impl(c)
    }
}

/// The 7 point size with a character size of 5x9 pixels.
pub type ProFont7Point<'a, C> = FontBuilder<'a, C, ProFont7PointConf>;

#[derive(Debug, Copy, Clone)]
pub enum ProFont9PointConf {}
impl FontBuilderConf for ProFont9PointConf {
    const FONT_IMAGE: &'static [u8] = include_bytes!("../data/ProFont9Point.raw");
    const CHAR_HEIGHT: u32 = 11;
    const CHAR_WIDTH: u32 = 6;
    const FONT_IMAGE_WIDTH: u32 = Self::CHAR_WIDTH * CHARS_PER_ROW;
    fn char_offset(c: char) -> u32 {
        char_offset_impl(c)
    }
}

/// The 9 point size with a character size of 6x11 pixels.
pub type ProFont9Point<'a, C> = FontBuilder<'a, C, ProFont9PointConf>;

#[derive(Debug, Copy, Clone)]
pub enum ProFont10PointConf {}
impl FontBuilderConf for ProFont10PointConf {
    const FONT_IMAGE: &'static [u8] = include_bytes!("../data/ProFont10Point.raw");
    const CHAR_HEIGHT: u32 = 13;
    const CHAR_WIDTH: u32 = 7;
    const FONT_IMAGE_WIDTH: u32 = Self::CHAR_WIDTH * CHARS_PER_ROW;
    fn char_offset(c: char) -> u32 {
        char_offset_impl(c)
    }
}

/// The 10 point size with a character size of 7x13 pixels.
pub type ProFont10Point<'a, C> = FontBuilder<'a, C, ProFont10PointConf>;

#[derive(Debug, Copy, Clone)]
pub enum ProFont12PointConf {}
impl FontBuilderConf for ProFont12PointConf {
    const FONT_IMAGE: &'static [u8] = include_bytes!("../data/ProFont12Point.raw");
    const CHAR_HEIGHT: u32 = 15;
    const CHAR_WIDTH: u32 = 8;
    const FONT_IMAGE_WIDTH: u32 = Self::CHAR_WIDTH * CHARS_PER_ROW;
    fn char_offset(c: char) -> u32 {
        char_offset_impl(c)
    }
}

/// The 12 point size with a character size of 8x15 pixels.
pub type ProFont12Point<'a, C> = FontBuilder<'a, C, ProFont12PointConf>;

#[derive(Debug, Copy, Clone)]
pub enum ProFont14PointConf {}
impl FontBuilderConf for ProFont14PointConf {
    const FONT_IMAGE: &'static [u8] = include_bytes!("../data/ProFont14Point.raw");
    const CHAR_HEIGHT: u32 = 18;
    const CHAR_WIDTH: u32 = 10;
    const FONT_IMAGE_WIDTH: u32 = Self::CHAR_WIDTH * CHARS_PER_ROW;
    fn char_offset(c: char) -> u32 {
        char_offset_impl(c)
    }
}

/// The 14 point size with a character size of 10x18 pixels.
pub type ProFont14Point<'a, C> = FontBuilder<'a, C, ProFont14PointConf>;

#[derive(Debug, Copy, Clone)]
pub enum ProFont18PointConf {}
impl FontBuilderConf for ProFont18PointConf {
    const FONT_IMAGE: &'static [u8] = include_bytes!("../data/ProFont18Point.raw");
    const CHAR_HEIGHT: u32 = 22;
    const CHAR_WIDTH: u32 = 12;
    const FONT_IMAGE_WIDTH: u32 = Self::CHAR_WIDTH * CHARS_PER_ROW;
    fn char_offset(c: char) -> u32 {
        char_offset_impl(c)
    }
}

/// The 18 point size with a character size of 12x22 pixels.
pub type ProFont18Point<'a, C> = FontBuilder<'a, C, ProFont18PointConf>;

#[derive(Debug, Copy, Clone)]
pub enum ProFont24PointConf {}
impl FontBuilderConf for ProFont24PointConf {
    const FONT_IMAGE: &'static [u8] = include_bytes!("../data/ProFont24Point.raw");
    const CHAR_HEIGHT: u32 = 30;
    const CHAR_WIDTH: u32 = 16;
    const FONT_IMAGE_WIDTH: u32 = Self::CHAR_WIDTH * CHARS_PER_ROW;
    fn char_offset(c: char) -> u32 {
        char_offset_impl(c)
    }
}

/// The 24 point size with a character size of 16x30 pixels.
pub type ProFont24Point<'a, C> = FontBuilder<'a, C, ProFont24PointConf>;
