use crate::patcher::Patcher;
use std::{fs, fs::File, io::prelude::*, path::PathBuf, io::BufReader};
use diffy_fork_filenames::{create_file_patch, PatchFormatter};

pub type Result<T, E = Error> = std::result::Result<T, E>;

pub(crate) struct Writer<'a> {
    path: PathBuf,
    patcher: &'a Patcher<'a>,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Invalid path")]
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

    pub(crate) fn patch_preview(&self, color: bool) -> Result<String, crate::writer::Error> {
        // TODO: Review error handling
        let file = File::open(self.path.clone()).expect("Error opening file");
        let buf = BufReader::new(file);
        let lines: Vec<String> = buf.lines()
            .map(|l| l.expect("Error getting line"))
            .collect();
        let original = lines.join("\n");
        let modified = match self.patcher.patch(lines) {
            Ok(replaced) => replaced,
            Err(err) => panic!("Error patching lines: {}", err), // FIXME:
        };
        let filename = match self.path.to_str() {
            Some(filename) => filename,
            None => return Err(Error::InvalidPath(self.path.clone())),
        };
        let original_filename = format!("a/{}", filename);
        let modified_filename = format!("b/{}", filename);
        let patch = create_file_patch(&original, &modified, original_filename.as_str(), modified_filename.as_str());
        let f = match color {
            true => PatchFormatter::new().with_color(),
            false => PatchFormatter::new(),
        };
        return Ok(f.fmt_patch(&patch).to_string());
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
            Err(_) => panic!("Error patching lines"), // FIXME:
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
