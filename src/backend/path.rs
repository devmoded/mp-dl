use std::path::PathBuf;

pub fn get_minecraft_dir() -> Option<PathBuf> {
    let base_dirs = directories::BaseDirs::new()?;

    let minecraft_dir = if cfg!(target_os = "windows") {
        base_dirs.config_dir().join(".minecraft")
    } else if cfg!(target_os = "linux") {
        base_dirs.home_dir().join(".minecraft")
    } else if cfg!(target_os = "macos") {
        base_dirs.data_dir().join("minecraft")
    } else {
        base_dirs.home_dir().join(".minecraft")
    };
    Some(minecraft_dir)
}
