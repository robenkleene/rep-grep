use crate::patcher::Patcher;
use std::{fs, fs::File, io::prelude::*, path::PathBuf, io::BufReader};
use diffy::create_patch;

pub type Result<T, E = Error> = std::result::Result<T, E>;

pub(crate) struct Writer<'a> {
    path: PathBuf,
    patcher: &'a Patcher<'a>,
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

impl<'a> Writer<'a> {
    pub(crate) fn new(path: PathBuf, patcher: &'a Patcher) -> Self {
        Self { path, patcher }
    }

    pub(crate) fn patch_preview(&self) -> Result<String, crate::patcher::Error> {
        // TODO: Review error handling
        let file = File::open(self.path.clone()).expect("Error opening file");
        let buf = BufReader::new(file);
        let lines = buf.lines()
            .map(|l| l.expect("Error getting line"))
            .collect();
        let replaced = match self.patcher.patch(lines) {
            Ok(replaced) => replaced,
            Err(_) => panic!("Unexpected error"), // FIXME:
        };
        let original = lines.join("\n")
        let patch = create_patch(original, modified);
        // FIXME: Add option for color
        let f = PatchFormatter::new().with_color();
        return f.fmt_patch(&patch);
    }

    pub(crate) fn write_file(&self) -> Result<()> {
        use memmap::{Mmap, MmapMut};
        use std::ops::DerefMut;

        let source = File::open(self.path.clone())?;
        let meta = fs::metadata(self.path.clone())?;
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

        target.persist(fs::canonicalize(self.path.clone())?)?;
        Ok(())
    }
}
