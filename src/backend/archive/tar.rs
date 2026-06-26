use std::fs::File;
use std::io::{Read, BufReader};
use std::marker::PhantomData;
use std::path::{Path, PathBuf};
use tar::Archive;
use flate2::read::GzDecoder;
use xz2::read::XzDecoder;
use zstd::stream::read::Decoder as ZstdDecoder;

use super::{ArchiveReader, EntryInfo, safe_extract_path};

pub trait TarDecoder: Read + Sized {
    fn from_file(file: File) -> anyhow::Result<Self>;
}

impl TarDecoder for GzDecoder<File> {
    fn from_file(file: File) -> anyhow::Result<Self> {
        Ok(GzDecoder::new(file))
    }
}

impl TarDecoder for XzDecoder<File> {
    fn from_file(file: File) -> anyhow::Result<Self> {
        Ok(XzDecoder::new(file))
    }
}

impl TarDecoder for ZstdDecoder<'static, BufReader<File>> {
    fn from_file(file: File) -> anyhow::Result<Self> {
        Ok(ZstdDecoder::new(file)?)
    }
}

pub struct TarReader<D: TarDecoder> {
    path: PathBuf,
    _phantom: PhantomData<D>,
}

impl<D: TarDecoder> TarReader<D> {
    pub fn open(path: &Path) -> anyhow::Result<Self> {
        Ok(Self {
            path: path.to_path_buf(),
            _phantom: PhantomData
        })
    }

    fn open_archive(&self) -> anyhow::Result<Archive<D>> {
        let file = File::open(&self.path)?;
        let decoder = D::from_file(file)?;
        Ok(Archive::new(decoder))
    }
}

impl<D: TarDecoder> ArchiveReader for TarReader<D> {
    fn entries(&mut self) -> anyhow::Result<Vec<super::EntryInfo>> {
        let mut archive = self.open_archive()?;
        let mut result = Vec::new();

        for entry in archive.entries()? {
            let entry = entry?;
            result.push(EntryInfo {
                name: entry.path()?.to_string_lossy().to_string(),
                is_dir: entry.header().entry_type().is_dir(),
                size: entry.header().size()?,
            });
        }
        Ok(result)
    }

    fn extract_all(&mut self, dest: &Path) -> anyhow::Result<()> {
        let mut archive = self.open_archive()?;
        archive.unpack(dest)?;
        Ok(())
    }

    fn extract_entry(&mut self, name: &str, dest: &Path) -> anyhow::Result<()> {
        let mut archive = self.open_archive()?;

        for entry in archive.entries()? {
            let mut entry = entry?;
            if entry.path()?.to_str() == Some(name) {
                let target = safe_extract_path(dest, name)?;
                if let Some(parent) = target.parent() {
                    std::fs::create_dir_all(parent)?;
                }
                entry.unpack(&target)?;
                return Ok(())
            }
        }
        anyhow::bail!("Файл {name} не найден в архиве")
    }
}

pub type TarGzReader = TarReader<GzDecoder<File>>;
pub type TarXzReader = TarReader<XzDecoder<File>>;
pub type TarZstReader = TarReader<ZstdDecoder<'static, BufReader<File>>>;
