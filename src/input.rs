use crate::{Replacer, Result, edit::Edit, patcher::Patcher, writer::Writer};
use std::io::prelude::*;
use std::fs::File;

pub(crate) struct App {
    replacer: Option<Replacer>
}

impl App {
    pub(crate) fn new(replacer: Option<Replacer>) -> Self {
        Self { replacer }
    }

    pub(crate) fn run(&self, preview: bool) -> Result<()> {
        {
            let mut buffer = Vec::with_capacity(256);
            let stdin = std::io::stdin();
            let mut handle = stdin.lock();

            let path_to_edit = Edit::parse(&handle);
            if preview {
                let stdout = std::io::stdout();
                let mut handle = stdout.lock();
                for (path, edits) in path_to_edit {
                    let patcher = Patcher::new(edits, self.replacer);
                    let writer = Writer::new(path, patcher);
                    if let Err(_) = Replacer::check_not_empty(File::open(path)?) {
                        return Ok(());
                    }
                    handle.write_all(writer.patch_preview())?;
                }
            } else {
                for (path, edits) in path_to_edit {
                    let patcher = Patcher::new(edits, self.replacer);
                    let writer = Writer::new(path, patcher);
                    if let Err(_) = Replacer::check_not_empty(File::open(path)?) {
                        return Ok(());
                    }
                    writer.write_file()
                }
            }
            Ok(())
        }
    }
}
