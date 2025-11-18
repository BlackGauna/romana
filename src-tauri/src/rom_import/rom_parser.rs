use std::path::{Path, PathBuf};

use crc_fast::checksum_file;
use walkdir::WalkDir;

// TODO: finish implementation
pub fn parse_roms_in_dir(path: PathBuf, comparison: u64) {
    let test = WalkDir::new(path)
        .sort_by_file_name()
        .into_iter()
        .filter_map(|f| f.ok())
        .filter(|f| f.file_type().is_file());

    for entry in test {
        let path = entry.path().to_str().expect("cannot get file string");
        if entry
            .path()
            .extension()
            .and_then(|s| s.to_str())
            .unwrap()
            .to_lowercase()
            == "7z"
        {
            checksum_7z(entry.path());
        } else {
            let checksum = checksum_file(crc_fast::CrcAlgorithm::Crc32IsoHdlc, path, None).unwrap();

            println!(
                "{:}: {:#X} {}",
                entry.file_name().display(),
                checksum,
                if checksum == comparison { "✓" } else { "╳" }
            );
        }
    }
}

/// Gets the CRC32 checksum of the first file inside the archive
fn checksum_7z(path: &Path) {
    let archive = sevenz_rust2::Archive::open(path).unwrap();

    for file in archive.files {
        println!("   {:#?}: {:#X}", file.name, file.crc);
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;

    #[test]
    fn test() {
        let hash_str = "29A78AF9";
        let hash = u64::from_str_radix(hash_str, 16).unwrap();

        let path = Path::new("/home/onur/Roms/snes");
        parse_roms_in_dir(path.to_path_buf(), hash);
    }
}
