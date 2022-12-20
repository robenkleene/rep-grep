use crate::patcher::Patcher;
use std::{fs, fs::File, io::prelude::*, path::PathBuf};

#[derive(Debug)]
pub(crate) struct Writer {
    file: PathBuf,
    patcher: Patcher,
}

impl Writer {
    pub(crate) fn new(file: PathBuf, patcher: Patcher) -> Self {
        Self { file, patcher }
    }

    pub(crate) fn patch_preview(&self) -> Result<()> {
        let stdout = std::io::stdout();
        let mut handle = stdout.lock();

        if let Err(_) = Replacer::check_not_empty(File::open(path)?)
        {
            return Ok(());
        }
        let file =
            unsafe { memmap::Mmap::map(&File::open(path)?)? };
        if self.replacer.has_matches(&file) {
            handle
                .write_all(&self.patch(&file))?;
            writeln!(handle)?;
        }

        Ok(())
    }

    pub(crate) fn write_file(&self) -> Result<()> {
        use memmap::{Mmap, MmapMut};
        use std::ops::DerefMut;

        if let Err(_) = Self::check_not_empty(File::open(path)?) {
            return Ok(());
        }

        let source = File::open(path)?;
        let meta = fs::metadata(path)?;
        let mmap_source = unsafe { Mmap::map(&source)? };
        let replaced = self.patch(&mmap_source);

        let target = tempfile::NamedTempFile::new_in(
            path.parent()
                .ok_or_else(|| Error::InvalidPath(path.to_path_buf()))?,
        )?;
        let file = target.as_file();
        file.set_len(replaced.len() as u64)?;
        file.set_permissions(meta.permissions())?;

        if !replaced.is_empty() {
            let mut mmap_target = unsafe { MmapMut::map_mut(&file)? };
            mmap_target.deref_mut().write_all(&replaced)?;
            mmap_target.flush_async()?;
        }

        drop(mmap_source);
        drop(source);

        target.persist(fs::canonicalize(path)?)?;
        Ok(())
    }
}
