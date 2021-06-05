//! A simple hello world example using the 12pt Pro Font font.

use embedded_graphics::{
    mono_font::MonoTextStyleBuilder,
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Baseline, Text, TextStyle},
};
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, Window};
use profont::PROFONT_12_POINT;

fn main() -> Result<(), core::convert::Infallible> {
    let mut display: SimulatorDisplay<BinaryColor> = SimulatorDisplay::new(Size::new(500, 128));

    let character_style = MonoTextStyleBuilder::new()
        .font(&PROFONT_12_POINT)
        // Uncomment to add strikethrough and/or underline
        // .strikethrough()
        // .underline()
        .text_color(BinaryColor::On)
        .build();
    let text_style = TextStyle::with_baseline(Baseline::Top);

    let test_text = "Hello world!";

    Text::with_text_style(test_text, Point::zero(), character_style, text_style)
        .draw(&mut display)?;

    let output_settings = OutputSettingsBuilder::new()
        // .theme(BinaryColorTheme::OledBlue)
        .scale(2)
        .build();

    Window::new("Pro Font hello world", &output_settings).show_static(&display);

    Ok(())
}
