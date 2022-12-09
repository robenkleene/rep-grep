mod cli;
mod error;
mod input;
pub(crate) mod edit;
mod patcher;
pub(crate) mod replacer;
pub(crate) mod utils;

pub(crate) use self::input::App;
pub(crate) use error::{Error, Result};
use replacer::Replacer;

fn main() -> Result<()> {
    use structopt::StructOpt;
    let options = cli::Options::from_args();

    App::new(
        Replacer::new(
            options.find,
            options.replace_with,
            options.literal_mode,
            options.flags,
            options.replacements,
        )?,
    )
    .run()?;
    Ok(())
}
