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
        .run()?;
    } else {
        App::new().run()?;
    }

    Ok(())
}
