use std::fs::File;
use std::io::{self, BufReader};
use std::path::{Path, PathBuf};
use tempdir::TempDir;
use tar::Archive;
use xz2::read::XzDecoder;



pub fn extract_tar_xz(file_from: &Path, file_to: &Path) -> io::Result<()> {
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


pub fn extract_zip(file_to_extract : &Path, destination : &Path) {

    let file_to_extract = File::open(file_to_extract).unwrap();
    zip_extract::extract(file_to_extract, destination, true).expect("could not extract")
}
