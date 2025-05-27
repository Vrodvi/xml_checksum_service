use std::fs::File;
use std::io::Read;
use std::path::Path;
use sha2::{Sha256, Digest};

pub struct HashCalculator;

impl HashCalculator {
    pub fn new() -> Self {
        Self
    }

    pub fn calculate_hash(&self, file_path: &Path) -> std::io::Result<String> {
        let mut file = File::open(file_path)?;
        let mut hasher = Sha256::new();
        let mut buffer = [0; 1024];
        
        while let Ok(n) = file.read(&mut buffer) {
            if n == 0 { break; }
            hasher.update(&buffer[..n]);
        }
        
        let hash = format!("{:x}", hasher.finalize());
        Ok(hash)
    }
} 