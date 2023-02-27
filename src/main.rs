mod cli;
mod error;
mod input;
mod edit;
mod patcher;
mod writer;
pub(crate) mod replacer;
pub(crate) mod utils;

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
        .run(!options.write, color)?;
    } else {
        App::new(None).run(!options.write, color)?;
    }

    Ok(())
}
