use crate::{Replacer, Result, Edit, Patcher, Writer};
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

            let pathToEdits = Edit::parse(&buffer);
            for (path, edits) in pathToEdits {
                let patcher = Patcher::new(edits, self.replacer);
                let writer = Writer::new(&path, patcher);
                handle.write_all(&if is_tty {
                    writer.patch_preview()
                } else {
                    writer.write_file()
                })?;
            }

            Ok(())
        }
    }
}
