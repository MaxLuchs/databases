use eyre::Result;
use std::fs::read_dir;
use std::path::Path;

pub fn list_all_folders(path: &Path) -> Result<Vec<Box<Path>>> {
    println!("list_all_folders : {:?}", path);
    let dirs = read_dir(path)?;
    Ok(dirs
        .map(|dir| dir.unwrap().path().into_boxed_path())
        .collect())
}
use std::fs;

pub fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}
