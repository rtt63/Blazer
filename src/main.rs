use md5::{Digest, Md5};
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::{Read, Seek, Write};

/// Calculating hash of the file using MD5 algorithm
/// It's not as safe as SHA256, but we don't care about security. It's lightweight, safe enough,
/// fits quite well to the task
///
/// Panic in case of read error
fn calc_hash(file: &mut File) -> String {
    let mut hasher = Md5::new();
    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer).unwrap();
    hasher.update(&buffer);

    let result = hasher.finalize();

    format!("{:x}", result)
}

const CACHE_FILE_NAME: &str = "./.blazercache.json";

fn read_or_create_cache() -> io::Result<String> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(CACHE_FILE_NAME)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    if contents.is_empty() {
        file.rewind()?;
        file.write_all(b"{}")?;
    }

    Ok(contents)
}

fn main() -> io::Result<()> {
    let cache_file = read_or_create_cache()?;
    println!("{}", cache_file);

    Ok(())
}
