use crate::patcher::Patcher;

#[derive(Debug)]
pub(crate) struct Writer {
    pub(crate) file: PathBuf,
    pub(crate) patcher: Patcher,
}

impl Writer {
    pub(crate) fn new(file: PathBuf, patcher: Patcher) -> Self {
        Self { file, patcher }
    }

    pub(crate) fn write(&self) -> Result<()> {
        let mut buffer = Vec::with_capacity(256);
        let mut file = File::open(&self.file)?;
        file.read_to_end(&mut buffer)?;

        let mut writer = File::create(&self.file)?;
        let mut offset = 0;

        for edit in &self.edits {
            let start = edit.start as usize;
            let end = edit.end as usize;

            writer.write_all(&buffer[offset..start])?;
            writer.write_all(&edit.replacement)?;
            offset = end;
        }

        writer.write_all(&buffer[offset..])?;

        Ok(())
    }

    pub(crate) fn patch_preview(&self, path: &Path) -> Result<()> {
        let stdout = std::io::stdout();
        let mut handle = stdout.lock();
        let print_path = paths.len() > 1;

        if let Err(_) = Replacer::check_not_empty(File::open(path)?)
        {
            return Ok(());
        }
        let file =
            unsafe { memmap::Mmap::map(&File::open(path)?)? };
        if self.replacer.has_matches(&file) {
            if print_path {
                writeln!(
                    handle,
                    "----- FILE {} -----",
                    path.display()
                )?;
            }

            handle
                .write_all(&self.patch(&file))?;
            writeln!(handle)?;
        }

        Ok(())
    }

    pub(crate) fn replace_file(&self, path: &Path) -> Result<()> {
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
