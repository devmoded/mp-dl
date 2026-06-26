pub mod zip;
pub mod tar;

use std::path::Path;
use std::path::PathBuf;

pub struct EntryInfo {
    pub name: String,
    pub is_dir: bool,
    pub size: u64,
}

pub trait ArchiveReader {
    fn entries(&mut self) -> anyhow::Result<Vec<EntryInfo>>;
    fn extract_all(&mut self, dest: &Path) -> anyhow::Result<()>;
    fn extract_entry(&mut self, name: &str, dest: &Path) -> anyhow::Result<()>;
}

pub fn open(path: &Path) -> anyhow::Result<Box<dyn ArchiveReader>> {
    let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

    if name.ends_with(".tar.gz") || name.ends_with(".tgz") {
        return Ok(Box::new(tar::TarGzReader::open(path)?));
    }
    if name.ends_with(".tar.xz") {
        return Ok(Box::new(tar::TarXzReader::open(path)?));
    }
    if name.ends_with(".tar.zst") {
        return Ok(Box::new(tar::TarZstReader::open(path)?));
    }

    match path.extension().and_then(|e| e.to_str()) {
        Some("zip") => Ok(Box::new(zip::ZipReader::open(path)?)),
        Some(ext) => anyhow::bail!("Не поддерживаемый формат {ext}"),
        None => anyhow::bail!("У файла не найдено расширение"),
    }
}

fn safe_extract_path(dest: &Path, entry_name: &str) -> anyhow::Result<PathBuf> {
    let mut target = dest.to_path_buf();
    for component in entry_name.split('/') {
        if component == ".." || component.is_empty() {
            continue;
        }
        target.push(component);
    }
    Ok(target)
}
