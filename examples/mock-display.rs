//! Uses the embedded-graphics `MockDisplay` to render output without requiring a display device.

use embedded_graphics::{
    mock_display::MockDisplay, mono_font::MonoTextStyle, pixelcolor::Rgb888, prelude::*, text::Text,
};
use profont::PROFONT_7_POINT;

fn main() -> Result<(), core::convert::Infallible> {
    let mut display = MockDisplay::new();

    let text_style = MonoTextStyle::new(&PROFONT_7_POINT, Rgb888::RED);

    Text::new("Hello world", Point::new(0, 7), text_style).draw(&mut display)?;

    // Print the mock display contents to the terminal
    println!("{:?}", display);

    Ok(())
}
