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
            let stdin = std::io::stdin();
            let handle = stdin.lock();

            match Edit::parse(handle) {
                Ok(path_to_edits) => {
                    if preview {
                        let stdout = std::io::stdout();
                        let mut handle = stdout.lock();
                        for (path, edits) in path_to_edits.into_iter() {
                            let patcher = Patcher::new(edits, self.replacer.as_ref());
                            if let Err(_) = Self::check_not_empty(File::open(&path)?) {
                                return Ok(())
                            }
                            let writer = Writer::new(path, patcher);
                            let text = match writer.patch_preview() {
                                Ok(text) => text,
                                Err(_) => continue, // FIXME:
                            };
                            handle.write_all(text.as_bytes());
                        }
                    } else {
                        for (path, edits) in path_to_edits {
                            let patcher = Patcher::new(edits, self.replacer.as_ref());
                            if let Err(_) = Self::check_not_empty(File::open(&path)?) {
                                return Ok(());
                            }
                            let writer = Writer::new(path, patcher);
                            writer.write_file();
                        }
                    }
                    Ok(())
                },
                Err(_) => {
                    return Ok(()); // FIXME:
                },
            }
        }
    }

    pub(crate) fn check_not_empty(mut file: File) -> Result<()> {
        let mut buf: [u8; 1] = Default::default();
        file.read_exact(&mut buf)?;
        Ok(())
    }
}
