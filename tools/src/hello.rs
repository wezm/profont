use std::ffi::OsString;

use embedded_graphics::{
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Baseline, Text, TextStyle},
};
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, Window};
use profont::PROFONT_12_POINT;

use crate::style;

pub fn main(args: Vec<OsString>) -> Result<(), anyhow::Error> {
    let (args, free_args) = match style::parse_args(args) {
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
                "hello",
                "A simple hello world example using the 12pt ProFont font"
            )
        );
        return Ok(());
    }

    let mut display: SimulatorDisplay<BinaryColor> = SimulatorDisplay::new(Size::new(500, 128));

    let character_style = style::binary_builder(&args).font(&PROFONT_12_POINT).build();
    let text_style = TextStyle::with_baseline(Baseline::Top);
    let test_text = free_args
        .first()
        .and_then(|arg| arg.to_str())
        .unwrap_or("Hello world!");

    Text::with_text_style(test_text, Point::zero(), character_style, text_style)
        .draw(&mut display)?;

    let output_settings = OutputSettingsBuilder::new()
        // TODO: Add theme to StyleArgs
        // .theme(BinaryColorTheme::OledBlue)
        .scale(u32::from(args.gui_scale))
        .build();

    Window::new("ProFont hello world", &output_settings).show_static(&display);

    Ok(())
}
