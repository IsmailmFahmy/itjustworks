use std::fs::File;
use std::io::{self, BufReader};
use std::path::{Path, PathBuf};
use xz2::read::XzDecoder;
use flate2::read::GzDecoder;
use tar::Archive;
use zip_extract;
use crate::packages::ExtractMethodEnum;






pub fn extract_package(method: &ExtractMethodEnum, downloaded_file_path :&PathBuf, extract_dir: &Path) -> Result<(), zip_extract::ZipError> {
use ExtractMethodEnum::*;

    let _ = match method {
        Zip => extract_zip(downloaded_file_path, extract_dir),
        Tarxz => extract_tar_xz(downloaded_file_path, extract_dir),
        Targz  => extract_tar_gz(downloaded_file_path, extract_dir),

    };

    Ok(())
}

fn extract_tar_xz(file_from: &Path, file_to: &Path) -> io::Result<()> {
    // Open the .tar.xz file
    let file = File::open(file_from)?;
    let buf_reader = BufReader::new(file);
    
    // Create an XZ decoder
    let xz_decoder = XzDecoder::new(buf_reader);
    
    // Create a tar archive from the XZ decoder
    let mut archive = Archive::new(xz_decoder);
    
    // Unpack the tar archive to the destination directory
    archive.unpack(file_to)?;
    
    Ok(())
}


fn extract_tar_gz(file_from: &Path, file_to: &Path) -> Result<(), io::Error> {

    let file = File::open(file_from)?;
    let mut archive = Archive::new(GzDecoder::new(file));

    // Extract the archive to the target directory
    for entry in archive.entries()? {
        let mut entry = entry?;
        let path = entry.path()?;
        
        let m = path.components().skip(1).collect::<std::path::PathBuf>();
        // Strip the top-level directory from the path
        let path = match m.to_str() {
            Some(p) => p,
            None => continue,
        };

        // Create the full path to the target directory
        let full_path = Path::new(file_to).join(path);

                // Create parent directories if needed
        if let Some(parent) = full_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        
        // Unpack the file
        entry.unpack(full_path)?;
    }
    Ok(())
}



fn extract_zip(file_to_extract : &Path, destination : &Path)-> io::Result<()>  {

    let file_to_extract = File::open(file_to_extract).unwrap();
    zip_extract::extract(file_to_extract, destination, true).expect("could not extract");
    Ok(())
}
