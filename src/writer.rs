use crate::patcher::Patcher;
use std::{fs, fs::File, io::prelude::*, path::PathBuf};

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug)]
pub(crate) struct Writer {
    file: PathBuf,
    patcher: Patcher,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Invalid line number")]
    InvalidPath(std::path::PathBuf),
}

impl Writer {
    pub(crate) fn new(path: PathBuf, patcher: Patcher) -> Self {
        Self { path, patcher }
    }

    pub(crate) fn patch_preview(&self) -> Result<String, crate::patcher::Error> {
        &self.patcher.patch(self.path)
    }

    pub(crate) fn write_file(&self) -> Result<()> {
        use memmap::{Mmap, MmapMut};
        use std::ops::DerefMut;

        if let Err(_) = Self::check_not_empty(File::open(self.path)?) {
            return Ok(());
        }

        let source = File::open(self.path)?;
        let meta = fs::metadata(self.path)?;
        let mmap_source = unsafe { Mmap::map(&source)? };
        let replaced = self.patch(&mmap_source);

        let target = tempfile::NamedTempFile::new_in(
            self.path.parent()
                .ok_or_else(|| Error::InvalidPath(self.path.to_path_buf()))?,
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

        target.persist(fs::canonicalize(self.path)?)?;
        Ok(())
    }

    pub(crate) fn check_not_empty(mut file: File) -> Result<()> {
        let mut buf: [u8; 1] = Default::default();
        file.read_exact(&mut buf)?;
        Ok(())
    }
}
