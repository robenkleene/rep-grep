mod cli;
mod edit;
mod error;
mod input;
mod less;
mod output;
mod patcher;
pub(crate) mod replacer;
pub(crate) mod utils;
mod writer;

pub(crate) use self::input::App;
pub(crate) use error::Result;
use replacer::Replacer;
use std::env;
use std::io::IsTerminal;
use std::process;

fn main() -> Result<()> {
    use clap::Parser;
    let options = cli::Options::parse();

    let is_tty = std::io::stdout().is_terminal();
    let color = if options.color {
        true
    } else if options.no_color {
        false
    } else {
        is_tty
    };

    let pager = env::var("REP_PAGER").ok();

    match (options.find, options.replace_with) {
        (Some(find), Some(replace_with)) => {
            App::new(Some(Replacer::new(
                find,
                replace_with,
                options.literal_mode,
                options.flags,
                options.replacements,
            )?))
            .run(!options.write, options.delete, color, options.stdout, pager)?;
        }
        (None, None) => {
            App::new(None).run(!options.write, options.delete, color, options.stdout, pager)?;
        }
        (Some(_), None) => {
            eprintln!("Error: missing replacement argument. Usage: rep <find> <replace>");
            process::exit(1);
        }
        (None, Some(_)) => {
            eprintln!("Error: missing search argument. Usage: rep <find> <replace>");
            process::exit(1);
        }
    }

    process::exit(0);
}
