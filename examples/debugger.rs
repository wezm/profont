//! Renders all characters in all sizes for debugging purposes.

use embedded_graphics::{
    mono_font::MonoTextStyleBuilder,
    pixelcolor::Rgb888,
    prelude::*,
    primitives::{Line, PrimitiveStyle},
    text::{Text, TextStyle},
};
use embedded_graphics_simulator::{OutputSettingsBuilder, SimulatorDisplay, Window};
use pico_args::Arguments;
use profont::*;

struct Args {
    show_help: bool,
    strikethrough: bool,
    underline: bool,
    gui_scale: u32,
}

const HELP_MESSAGE: &str = "\
Profont debugger. Renders all characters in all sizes for debugging purposes.

USAGE:
    debugger [OPTIONS]

FLAGS:
  -h, --help            Prints help information

OPTIONS:
  -s, --strikethrough   Enables strikethrough style
  -u, --underline       Enables underline style
  --no-gui-scaling      Disable GUI scaling by factor 2
";

fn parse_args() -> Result<Args, pico_args::Error> {
    let mut args = Arguments::from_env();

    let show_help = args.contains(["-h", "--help"]);
    let strikethrough = args.contains(["-s", "--strikethrough"]);
    let underline = args.contains(["-u", "--underline"]);
    let no_gui_scaling = args.contains("--no-gui-scaling");

    if args.finish().is_empty() {
        Ok(Args {
            show_help,
            strikethrough,
            underline,
            gui_scale: if no_gui_scaling { 1 } else { 2 },
        })
    } else {
        Err(pico_args::Error::ArgumentParsingFailed {
            cause: "Unknown arguments. Use '--help' for help on usage.".to_string(),
        })
    }
}

fn base_character_style_builder(args: &Args) -> MonoTextStyleBuilder<Rgb888> {
    let mut style = MonoTextStyleBuilder::new().text_color(Rgb888::WHITE);

    if args.strikethrough {
        style = style.strikethrough_with_color(Rgb888::CSS_TOMATO);
    }
    if args.underline {
        style = style.underline_with_color(Rgb888::CSS_CORNFLOWER_BLUE);
    }

    style
}

fn main() -> Result<(), std::convert::Infallible> {
    let args = match parse_args() {
        Ok(args) => args,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };

    if args.show_help {
        println!("{}", HELP_MESSAGE);
        return Ok(());
    }

    let mut display: SimulatorDisplay<Rgb888> = SimulatorDisplay::new(Size::new(1000, 500));
    let character_style = base_character_style_builder(&args);
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

    let output_settings = OutputSettingsBuilder::new().scale(args.gui_scale).build();
    Window::new("Pro Font debugger", &output_settings).show_static(&display);

    Ok(())
}
