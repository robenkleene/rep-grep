use crate::{Replacer, Result, edit::Edit, patcher::Patcher, writer::Writer, output::OutputType};
use std::io::prelude::*;
use std::fs::File;

pub(crate) struct App {
    replacer: Option<Replacer>
}

impl App {
    pub(crate) fn new(replacer: Option<Replacer>) -> Self {
        Self { replacer }
    }

    pub(crate) fn run(&self, preview: bool, color: bool, pager: Option<String>) -> Result<()> {
        {
            let stdin = std::io::stdin();
            let handle = stdin.lock();

            // FIXME: Instantiating `output_type` and `write` should only happen if `preview` is true
            let mut output_type = match OutputType::for_pager(pager, true) {
                Ok(output_type) => output_type,
                Err(_) => return Ok(()), // FIXME:
            };

            let write = match output_type.handle() {
                Ok(write) => write,
                Err(_) => return Ok(()), // FIXME:
            };

            match Edit::parse(handle) {
                Ok(path_to_edits) => {
                    if preview {
                        for (path, edits) in path_to_edits {
                            let patcher = Patcher::new(edits, self.replacer.as_ref());
                            if let Err(_) = Self::check_not_empty(File::open(&path)?) {
                                continue // FIXME:
                            }
                            let writer = Writer::new(path.to_path_buf(), &patcher);
                            let text = match writer.patch_preview(color) {
                                Ok(text) => text,
                                Err(_) => continue, // FIXME:
                            };

                            write!(write, "{}", text)?;
                        }
                    } else {
                        for (path, edits) in path_to_edits {
                            let patcher = Patcher::new(edits, self.replacer.as_ref());
                            if let Err(_) = Self::check_not_empty(File::open(&path)?) {
                                return Ok(()); // FIXME:
                            }
                            let writer = Writer::new(path, &patcher);
                            if let Err(_) = writer.write_file() {
                                return Ok(()); // FIXME:
                            }
                        }
                    }
                },
                Err(_) => {
                    return Ok(()); // FIXME:
                },
            }
            drop(write);
        }
        Ok(())
    }

    pub(crate) fn check_not_empty(mut file: File) -> Result<()> {
        let mut buf: [u8; 1] = Default::default();
        file.read_exact(&mut buf)?;
        Ok(())
    }
}
