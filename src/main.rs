mod cli;
mod error;
mod input;
mod edit;
mod patcher;
mod writer;
mod output;
// FIXME: Look at `pub(crate)` calls are all these necessary?
pub(crate) mod replacer;
pub(crate) mod utils;
mod less;

use std::process;
use std::env;
pub(crate) use self::input::App;
pub(crate) use error::Result;
use replacer::Replacer;

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
        App::new(
            Some(Replacer::new(
                find,
                replace_with,
                options.literal_mode,
                options.flags,
                options.replacements,
            )?),
        )
        .run(!options.write, options.delete, color, pager)?;
    } else {
        App::new(None).run(!options.write, options.delete, color, pager)?;
    }

    process::exit(0);
}
