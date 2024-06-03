use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use directories::UserDirs;

pub struct UserDocumentsWriter;

impl FileWriter for UserDocumentsWriter {
    fn write(path: &Path, content: &str) -> std::io::Result<String> {
        if let Some(user_dirs) = UserDirs::new() {
            if let Some(document_path) = user_dirs.document_dir() {
                let full_path = document_path.join(path);
                return write_file(&full_path, content);
            }
        }
        write_file(path, content)
    }
}
trait FileWriter {
    fn write(path: &Path, content: &str) -> std::io::Result<String> {
        write_file(path, content)
    }
}

pub fn write_file(path: &Path, content: &str) -> std::io::Result<String> {
    let mut file = File::create(&path)?;
    file.write_all(content.as_bytes())?;
    let result = format!("file saved to '{:?}'", path);
    Ok(result)
}
pub fn write_readme(file_name: &str, content: &str) -> std::io::Result<String> {
    let file_path = format!("{}.md", file_name);
    return UserDocumentsWriter::write(Path::new(file_path.as_str()), content);
}
