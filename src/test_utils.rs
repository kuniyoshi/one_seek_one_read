#[cfg(test)]
pub mod test_utils {
    use std::fs::{self, File};
    use std::io::Write;
    use std::path::Path;
    use crate::util::get_hash;

    pub fn setup_test_data() -> std::io::Result<()> {
        fs::create_dir_all("data")?;
        fs::create_dir_all("data/resource.d")?;

        let test_files = vec![
            ("data/resource.d/file1.txt", b"Hello, World!" as &[u8]),
            ("data/resource.d/file2.txt", b"Test content" as &[u8]),
            ("data/resource.d/file3.txt", b"Another file" as &[u8]),
        ];
        
        // Ensure parent directory exists
        fs::create_dir_all("data/resource.d")?;

        let mut archive_data = Vec::new();
        let mut index_content = String::new();
        let mut _offset = 0;

        for (path, content) in &test_files {
            let mut file = File::create(path)?;
            file.write_all(content)?;

            let hash = get_hash(&content.to_vec());
            let size = content.len();
            
            index_content.push_str(&format!("{}\t{}\t{}\n", path, size, hash));
            
            archive_data.extend_from_slice(content);
            _offset += size;
        }

        let mut archive_file = File::create("data/archive.data")?;
        archive_file.write_all(&archive_data)?;

        let mut index_file = File::create("data/resource.index")?;
        index_file.write_all(index_content.as_bytes())?;

        Ok(())
    }

    pub fn cleanup_test_data() -> std::io::Result<()> {
        if Path::new("data").exists() {
            fs::remove_dir_all("data")?;
        }
        Ok(())
    }
}