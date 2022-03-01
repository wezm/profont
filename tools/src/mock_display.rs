use std::ffi::OsString;

use embedded_graphics::{
    mock_display::MockDisplay, mono_font::MonoTextStyle, pixelcolor::Rgb888, prelude::*, text::Text,
};
use profont::PROFONT_7_POINT;

const HELP_MESSAGE: &str = "\
profont mock-display

Uses the embedded-graphics `MockDisplay` to render output without requiring
a display device.

USAGE:
    mock-display [OPTIONS] [TEXT]

FLAGS:
  -h, --help    Prints help information
";

pub fn main(args: Vec<OsString>) -> Result<(), anyhow::Error> {
    if args.iter().any(|arg| arg.to_str() == Some("--help")) {
        println!("{}", HELP_MESSAGE);
        return Ok(());
    }

    let mut display = MockDisplay::new();

    let text_style = MonoTextStyle::new(&PROFONT_7_POINT, Rgb888::RED);
    let text = args
        .first()
        .and_then(|arg| arg.to_str())
        .unwrap_or("Hello world");
    Text::new(text, Point::new(0, 7), text_style).draw(&mut display)?;

    // Print the mock display contents to the terminal
    println!("{:?}", display);

    Ok(())
}
