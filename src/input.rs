use crate::{Replacer, Result, Edit, Patcher, Writer};
use std::io::prelude::*;

pub(crate) struct App {
    replacer: Option<Replacer>
}

impl App {
    pub(crate) fn new(replacer: Option<Replacer>) -> Self {
        Self { replacer }
    }

    pub(crate) fn run(&self, preview: bool) -> Result<()> {
        let is_tty = atty::is(atty::Stream::Stdout);

        {
            let mut buffer = Vec::with_capacity(256);
            let stdin = std::io::stdin();
            let mut handle = stdin.lock();
            handle.read_to_end(&mut buffer)?;

            let pathToEdit = Edit::parse(&buffer);
            if preview {
                let stdout = std::io::stdout();
                let mut handle = stdout.lock();
                for (path, edits) in pathToEdit {
                    handle.write_all(writer.patch_preview(is_tty))?;
                }
            } else {
                let patcher = Patcher::new(edits, self.replacer);
                let writer = Writer::new(&path, patcher);
                writer.write_file()
            }
            Ok(())
        }
    }
}
