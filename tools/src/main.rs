mod debugger;
mod generate;
mod hello;
mod mock_display;
mod style;

use anyhow::{anyhow, bail};

pub fn main() -> Result<(), anyhow::Error> {
    let mut args = std::env::args_os().skip(1);
    let cmd = args.next().ok_or_else(|| {
        usage();
        anyhow!("a command must be specified")
    })?;
    let args = args.collect::<Vec<_>>();

    match cmd.to_str() {
        Some("debugger") => debugger::main(args),
        Some("generate") => generate::main(args),
        Some("hello") => hello::main(args),
        Some("mock-display") => mock_display::main(args),
        _ => {
            usage();
            bail!("invalid command")
        }
    }
}

fn usage() {
    let info = "\
Usage: profont <command>

Where <command> is one of:

  debugger        Render sample text in all sizes
  generate        Regenerate the font
  hello           Show hello world text on simulated display
  mock-display    Render text to the terminal

Each command may also have its own options, and accepts --help.
";
    println!("{}", info);
}
