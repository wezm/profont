//! # Example: Custom font
//!
//! Shows how to implement a custom `SevenSegmentFont` font using the `MonoFontBuilder` struct. This
//! font renders numbers only and emulates a classic 7 segment display.

use embedded_graphics::{
    image::ImageRaw,
    mono_font::{MonoTextStyle, MonoTextStyleBuilder},
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Alignment, Baseline, Text, TextStyle, TextStyleBuilder},
};
use embedded_graphics_simulator::{
    BinaryColorTheme, OutputSettingsBuilder, SimulatorDisplay, Window,
};
use profont::PROFONT_7_POINT;

fn main() -> Result<(), core::convert::Infallible> {
    let mut display: SimulatorDisplay<BinaryColor> = SimulatorDisplay::new(Size::new(500, 128));

    // let character_style = MonoTextStyle::new(&PROFONT_7_POINT, BinaryColor::On);
    let character_style = MonoTextStyleBuilder::new()
        .font(&PROFONT_7_POINT)
        .strikethrough()
        .underline()
        .text_color(BinaryColor::On)
        .build();
    let text_style = TextStyle::with_baseline(Baseline::Top);

    let test_text  = "Hello world!\n¡¢£¤¥¦§¨©ª«¬­®¯°±²³´µ¶·¸¹º»¼½¾¿ÀÁÂÃÄÅÆÇÈÉÊËÌÍÎÏÐÑÒÓÔÕÖ×ØÙÚÛÜÝÞßàáâãäåæçèéêëìíîïðñòóôõö÷øùúûüýþÿ";

    Text::with_text_style(
        // "123\n456\nÿ",
        test_text,
        // Point::new(10, 10),
        Point::zero(),
        character_style,
        text_style,
    )
    .draw(&mut display)?;

    let output_settings = OutputSettingsBuilder::new()
        // .theme(BinaryColorTheme::OledBlue)
        .scale(2)
        .build();
    Window::new("Custom font", &output_settings).show_static(&display);

    Ok(())
}
