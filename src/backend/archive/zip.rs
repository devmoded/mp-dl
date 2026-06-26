use std::path::Path;
use std::fs::File;

use super::{ArchiveReader, EntryInfo, safe_extract_path};

pub struct ZipReader {
    content: zip::ZipArchive<File>
}

impl ZipReader {
    pub fn open(path: &Path) -> anyhow::Result<Self> {
        let file = File::open(path)?;
        Ok(Self { content: zip::ZipArchive::new(file)? })
    }
}

impl ArchiveReader for ZipReader {
    fn entries(&mut self) -> anyhow::Result<Vec<super::EntryInfo>> {
        let mut result = Vec::new();
        for i in 0..self.content.len() {
            let entry = self.content.by_index(i)?;
            result.push(EntryInfo {
                name: entry.name().to_string(),
                is_dir: entry.is_dir(),
                size: entry.size(),
            });
        }
        Ok(result)
    }

    fn extract_all(&mut self, dest: &Path) -> anyhow::Result<()> {
        for i in 0..self.content.len() {
            let mut entry = self.content.by_index(i)?;
            let entry_path = safe_extract_path(dest, entry.name())?;

            if entry.is_dir() {
                std::fs::create_dir_all(&entry_path)?;
            } else {
                if let Some(parent) = entry_path.parent() {
                    std::fs::create_dir_all(parent)?;
                }
                let mut out_file = File::create(&entry_path)?;
                std::io::copy(&mut entry, &mut out_file)?;
            }
        }
        Ok(())
    }

    fn extract_entry(&mut self, name: &str, dest: &Path) -> anyhow::Result<()> {
        let mut entry = self.content.by_name(name)?;
        let target = safe_extract_path(dest, name)?;

        if entry.is_dir() {
            std::fs::create_dir_all(&target)?;
        } else {
            if let Some(parent) = target.parent() {
                std::fs::create_dir_all(parent)?;
            }
            let mut out_file = File::create(&target)?;
            std::io::copy(&mut entry, &mut out_file)?;
        }
        Ok(())
    }
}
