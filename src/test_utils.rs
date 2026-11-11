use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

use crate::util::get_hash;

pub struct TestData {
    directory: PathBuf,
}

impl TestData {
    pub fn new() -> io::Result<Self> {
        let directory =
            std::env::temp_dir().join(format!("one_seek_one_read-{:032x}", rand::random::<u128>()));
        fs::create_dir(&directory)?;
        let data = Self { directory };

        let resources = data.directory.join("resource.d");
        fs::create_dir(&resources)?;
        let mut archive = fs::File::create(data.archive_path())?;
        let mut index = fs::File::create(data.index_path())?;

        let files: [(&str, &[u8]); 3] = [
            ("file1.txt", b"Hello, World!"),
            ("file2.txt", b"Test content"),
            ("file3.txt", b"Another file"),
        ];
        for (name, content) in files {
            let path = resources.join(name);
            fs::write(&path, content)?;
            writeln!(
                index,
                "{}\t{}\t{}",
                path.display(),
                content.len(),
                get_hash(&content.to_vec())
            )?;
            archive.write_all(content)?;
        }

        Ok(data)
    }

    pub fn index_path(&self) -> String {
        self.directory.join("resource.index").display().to_string()
    }

    pub fn archive_path(&self) -> String {
        self.directory.join("archive.data").display().to_string()
    }
}

impl Drop for TestData {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.directory);
    }
}
