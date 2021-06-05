//! Renders all characters in all sizes for debugging purposes.

use embedded_graphics::{
    mono_font::MonoTextStyleBuilder,
    pixelcolor::Rgb888,
    prelude::*,
    primitives::{Line, PrimitiveStyle},
    text::{Text, TextStyle},
};
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, Window};
use profont::*;

fn main() -> Result<(), core::convert::Infallible> {
    let mut display: SimulatorDisplay<Rgb888> = SimulatorDisplay::new(Size::new(1000, 500));

    let character_style = MonoTextStyleBuilder::new()
        // Uncomment to add strikethrough and/or underline to all sizes.
        // .strikethrough_with_color(Rgb888::CSS_TOMATO)
        // .underline_with_color(Rgb888::CSS_CORNFLOWER_BLUE)
        .text_color(Rgb888::WHITE);

    let text_style = TextStyle::default();

    let sizes = [
        PROFONT_7_POINT,
        PROFONT_9_POINT,
        PROFONT_10_POINT,
        PROFONT_12_POINT,
        PROFONT_14_POINT,
        PROFONT_18_POINT,
        PROFONT_24_POINT,
    ];

    let mut position = Point::new(10, 10);

    for size in sizes.iter() {
        let character_style = character_style.font(size).build();

        let test_text  = format!("Hello world! jpyJPY{}\n¡¢£¤¥¦§¨©ª«¬­®¯°±²³´µ¶·¸¹º»¼½¾¿\nÀÁÂÃÄÅÆÇÈÉÊËÌÍÎÏÐÑÒÓÔÕÖ×ØÙÚÛÜÝÞ\nßàáâãäåæçèéêëìíîïðñòóôõö÷øùúûüýþÿ\u{ffff}", size.character_size);

        // Draw the font baseline behind the first line of text
        Line::new(
            position.y_axis(),
            position.y_axis() + display.bounding_box().size.x_axis(),
        )
        .into_styled(PrimitiveStyle::with_stroke(Rgb888::RED, 1))
        .draw(&mut display)?;

        Text::with_text_style(&test_text, position, character_style, text_style)
            .draw(&mut display)?;

        position += size.character_size.y_axis() * test_text.lines().count() as u32;
    }

    let output_settings = OutputSettingsBuilder::new().scale(2).build();
    Window::new("Pro Font debugger", &output_settings).show_static(&display);

    Ok(())
}
