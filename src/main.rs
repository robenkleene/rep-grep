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
use std::process;

fn main() -> Result<()> {
    use structopt::StructOpt;
    let options = cli::Options::from_args();

    let is_tty = atty::is(atty::Stream::Stdout);
    let color = if options.color {
        true
    } else if options.no_color {
        false
    } else if is_tty {
        true
    } else {
        false
    };

    let pager = env::var("REP_PAGER").ok();

    if let (Some(find), Some(replace_with)) = (options.find, options.replace_with) {
        App::new(Some(Replacer::new(
            find,
            replace_with,
            options.literal_mode,
            options.flags,
            options.replacements,
        )?))
        .run(!options.write, options.delete, color, options.stdout, pager)?;
    } else {
        App::new(None).run(!options.write, options.delete, color, options.stdout, pager)?;
    }

    process::exit(0);
}
