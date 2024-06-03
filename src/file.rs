use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

const OUTPUT_DIR: &str = "./output";

pub fn write_to_file(file_name: &str, content: &str) -> std::io::Result<()> {
    let path = Path::new(OUTPUT_DIR).join(&Path::new(file_name));

    let mut file = File::create(&path)?;
    file.write_all(content.as_bytes())?;
    println!("file saved to '{:?}'", path);
    Ok(())
}

pub fn write_readme(file_name: &str, content: &str) -> std::io::Result<()> {
    return write_to_file(format!("markdown/{}.md", file_name).as_str(), content);
}
