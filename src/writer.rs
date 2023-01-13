use crate::patcher::Patcher;
use std::{fs, fs::File, io::prelude::*, path::PathBuf, io::BufReader};
pub type Result<T, E = Error> = std::result::Result<T, E>;

pub(crate) struct Writer<'a> {
    path: PathBuf,
    patcher: Patcher<'a>,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Invalid line number")]
    InvalidPath(std::path::PathBuf),
    #[error(transparent)]
    File(#[from] std::io::Error),
    #[error("failed to move file: {0}")]
    TempfilePersist(#[from] tempfile::PersistError),
}

impl Writer<'_> {
    pub(crate) fn new(path: PathBuf, patcher: Patcher) -> Self {
        Self { path, patcher }
    }

    pub(crate) fn patch_preview(&self) -> Result<String, crate::patcher::Error> {
        // TODO: Review error handling
        let file = File::open(self.path).expect("Error opening file");
        let buf = BufReader::new(file);
        let lines = buf.lines()
            .map(|l| l.expect("Error getting line"))
            .collect();
        Ok(self.patcher.patch(lines)?)
    }

    pub(crate) fn write_file(&self) -> Result<()> {
        use memmap::{Mmap, MmapMut};
        use std::ops::DerefMut;

        let source = File::open(self.path)?;
        let meta = fs::metadata(self.path)?;
        let mmap_source = unsafe { Mmap::map(&source)? };
        let lines = mmap_source.lines()
            .map(|l| l.expect("Error getting line"))
            .collect();
        let replaced = match self.patcher.patch(lines) {
            Ok(replaced) => replaced,
            Err(_) => panic!("Unexpected error"), // FIXME:
        };

        let target = tempfile::NamedTempFile::new_in(
            self.path.parent()
                .ok_or_else(|| Error::InvalidPath(self.path.to_path_buf()))?,
        )?;
        let file = target.as_file();
        file.set_len(replaced.len() as u64)?;
        file.set_permissions(meta.permissions())?;

        if !replaced.is_empty() {
            let mut mmap_target = unsafe { MmapMut::map_mut(&file)? };
            mmap_target.deref_mut().write_all(&replaced.as_bytes())?;
            mmap_target.flush_async()?;
        }

        drop(mmap_source);
        drop(source);

        target.persist(fs::canonicalize(self.path)?)?;
        Ok(())
    }
}
