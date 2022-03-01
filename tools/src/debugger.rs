use std::ffi::OsString;

use embedded_graphics::{
    pixelcolor::Rgb888,
    prelude::*,
    primitives::{Line, PrimitiveStyle},
    text::{Text, TextStyle},
};
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, Window};
use profont::*;

use crate::style;

pub fn main(args: Vec<OsString>) -> Result<(), anyhow::Error> {
    let (args, _free_args) = match style::parse_args(args) {
        Ok(args) => args,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };

    if args.show_help {
        println!(
            "{}",
            style::help(
                "debugger",
                "Renders all characters in all sizes for debugging purposes"
            )
        );
        return Ok(());
    }

    let mut display: SimulatorDisplay<Rgb888> = SimulatorDisplay::new(Size::new(800, 640));
    let character_style = style::builder(&args);
    let text_style = TextStyle::default();

    let sizes = [
        (PROFONT_7_POINT, 7),
        (PROFONT_9_POINT, 9),
        (PROFONT_10_POINT, 10),
        (PROFONT_12_POINT, 12),
        (PROFONT_14_POINT, 14),
        (PROFONT_18_POINT, 18),
        (PROFONT_24_POINT, 24),
    ];

    let mut position = Point::new(10, 10);

    for (font, size) in sizes.iter() {
        let character_style = character_style.font(font).build();

        let test_text  = format!("ProFont {} jpyJPY {} HiMw!\n¡¢£¤¥¦§¨©ª«¬­®¯°±²³´µ¶·¸¹º»¼½¾¿\nÀÁÂÃÄÅÆÇÈÉÊËÌÍÎÏÐÑÒÓÔÕÖ×ØÙÚÛÜÝÞ\nßàáâãäåæçèéêëìíîïðñòóôõö÷øùúûüýþÿ\u{ffff}", size, font.character_size);

        // Draw the font baseline behind the first line of text
        Line::new(
            position.y_axis(),
            position.y_axis() + display.bounding_box().size.x_axis(),
        )
        .into_styled(PrimitiveStyle::with_stroke(Rgb888::RED, 1))
        .draw(&mut display)?;

        Text::with_text_style(&test_text, position, character_style, text_style)
            .draw(&mut display)?;

        position += font.character_size.y_axis() * test_text.lines().count() as u32;
        position.y += font.character_size.height as i32 * 2;
    }

    let output_settings = OutputSettingsBuilder::new()
        .scale(u32::from(args.gui_scale))
        .build();
    Window::new("ProFont Debugger", &output_settings).show_static(&display);

    Ok(())
}
