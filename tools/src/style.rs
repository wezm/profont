use std::ffi::OsString;

use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::{mono_font::MonoTextStyleBuilder, pixelcolor::Rgb888, prelude::*};
use pico_args::Arguments;

pub struct StyleArgs {
    pub show_help: bool,
    pub strikethrough: bool,
    pub underline: bool,
    pub gui_scale: u8,
}

pub fn parse_args(args: Vec<OsString>) -> Result<(StyleArgs, Vec<OsString>), pico_args::Error> {
    let mut args = Arguments::from_vec(args);

    let show_help = args.contains(["-h", "--help"]);
    let strikethrough = args.contains(["-s", "--strikethrough"]);
    let underline = args.contains(["-u", "--underline"]);
    let no_gui_scaling = args.contains("--no-gui-scaling");

    Ok((
        StyleArgs {
            show_help,
            strikethrough,
            underline,
            gui_scale: if no_gui_scaling { 1 } else { 2 },
        },
        args.finish(),
    ))
}

pub fn builder(args: &StyleArgs) -> MonoTextStyleBuilder<Rgb888> {
    let mut style = MonoTextStyleBuilder::new().text_color(Rgb888::WHITE);

    if args.strikethrough {
        style = style.strikethrough_with_color(Rgb888::CSS_TOMATO);
    }
    if args.underline {
        style = style.underline_with_color(Rgb888::CSS_CORNFLOWER_BLUE);
    }

    style
}

pub fn binary_builder(args: &StyleArgs) -> MonoTextStyleBuilder<BinaryColor> {
    let mut style = MonoTextStyleBuilder::new().text_color(BinaryColor::On);

    if args.strikethrough {
        style = style.strikethrough()
    }
    if args.underline {
        style = style.underline()
    }

    style
}

pub fn help(command_name: &str, description: &str) -> String {
    format!(
        "\
profont {}

{}.

USAGE:
    debugger [OPTIONS]

FLAGS:
  -h, --help            Prints help information

OPTIONS:
  -s, --strikethrough   Enables strikethrough style
  -u, --underline       Enables underline style
      --no-gui-scaling  Disable GUI scaling by factor 2
",
        command_name, description
    )
}
