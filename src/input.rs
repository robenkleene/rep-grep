use crate::{edit::Edit, output::OutputType, patcher::Patcher, writer::Writer, Replacer, Result};
use std::fs::File;
use std::io::prelude::*;

pub(crate) struct App {
    replacer: Option<Replacer>,
}

impl App {
    pub(crate) fn new(replacer: Option<Replacer>) -> Self {
        Self { replacer }
    }

    pub(crate) fn run(
        &self,
        preview: bool,
        delete: bool,
        color: bool,
        stdout: bool,
        pager: Option<String>,
    ) -> Result<()> {
        {
            let stdin = std::io::stdin();
            let handle = stdin.lock();

            let path_to_edits = Edit::parse(handle)?;

            if preview {
                let mut output_type = if stdout {
                    OutputType::stdout()
                } else {
                    OutputType::for_pager(pager, true)?
                };

                let write = output_type.handle()?;

                for (path, edits) in path_to_edits {
                    let patcher = Patcher::new(edits, self.replacer.as_ref());
                    let file = match File::open(&path) {
                        Ok(f) => f,
                        Err(e) => {
                            eprintln!("{}: {}", path.display(), e);
                            continue;
                        }
                    };
                    if Self::check_not_empty(file).is_err() {
                        continue;
                    }
                    let writer = Writer::new(path.to_path_buf(), &patcher);
                    let text = match writer.patch_preview(color, delete) {
                        Ok(text) => text,
                        Err(e) => {
                            eprintln!("{}", e);
                            continue;
                        }
                    };

                    write!(write, "{}", text)?;
                }
                drop(output_type);
            } else {
                for (path, edits) in path_to_edits {
                    let patcher = Patcher::new(edits, self.replacer.as_ref());
                    let file = match File::open(&path) {
                        Ok(f) => f,
                        Err(e) => {
                            eprintln!("{}: {}", path.display(), e);
                            continue;
                        }
                    };
                    if Self::check_not_empty(file).is_err() {
                        continue;
                    }
                    let writer = Writer::new(path, &patcher);
                    writer.write_file(delete)?;
                }
            }
        }
        Ok(())
    }

    pub(crate) fn check_not_empty(mut file: File) -> Result<()> {
        let mut buf: [u8; 1] = Default::default();
        file.read_exact(&mut buf)?;
        Ok(())
    }
}
