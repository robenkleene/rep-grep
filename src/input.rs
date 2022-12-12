use crate::{Replacer, Result};
use std::io::prelude::*;

pub(crate) struct App {
    replacer: Option<Replacer>
}

impl App {
    pub(crate) fn new(replacer: Option<Replacer>) -> Self {
        Self { replacer }
    }

    pub(crate) fn run(&self) -> Result<()> {
        let is_tty = atty::is(atty::Stream::Stdout);
        {
            let mut buffer = Vec::with_capacity(256);
            let stdin = std::io::stdin();
            let mut handle = stdin.lock();
            handle.read_to_end(&mut buffer)?;

            let stdout = std::io::stdout();
            let mut handle = stdout.lock();

            if let Some(replacer) = &self.replacer {
                handle.write_all(&if is_tty {
                    replacer.replace_preview(&buffer)
                } else {
                    replacer.replace(&buffer)
                })?;
            }

            Ok(())
        }
    }
}
