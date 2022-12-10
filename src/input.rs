use crate::{Replacer, Result};
use std::io::prelude::*;

pub(crate) struct App {
    replacer: Replacer
}

impl App {
    pub(crate) fn new(replacer: Replacer) -> Self {
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

            handle.write_all(&if is_tty {
                self.replacer.replace_preview(&buffer)
            } else {
                self.replacer.replace(&buffer)
            })?;

            Ok(())
        }
    }
}
