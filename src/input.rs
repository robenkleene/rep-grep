use crate::{Replacer, Result, edit::Edit, patcher::Patcher, writer::Writer};
use std::io::prelude::*;

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
            handle.read_to_end(&mut buffer)?;

            let path_to_edit = Edit::parse(&buffer);
            let patcher = Patcher::new(edits, self.replacer);
            let writer = Writer::new(&file, patcher);
            if preview {
                let stdout = std::io::stdout();
                let mut handle = stdout.lock();
                for (path, edits) in path_to_edit {
                    if let Err(_) = Replacer::check_not_empty(File::open(path)?) {
                        return Ok(());
                    }
                    let file =
                        unsafe { memmap::Mmap::map(&File::open(path)?)? };
                    handle.write_all(writer.patch_preview())?;
                }
            } else {
                for (path, edits) in path_to_edit {
                    if let Err(_) = Replacer::check_not_empty(File::open(path)?) {
                        return Ok(());
                    }
                    let file =
                        unsafe { memmap::Mmap::map(&File::open(path)?)? };
                    writer.write_file()
                }
            }
            Ok(())
        }
    }
}
