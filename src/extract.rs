use std::fs::File;
use std::io::{self, BufReader};
use std::path::{Path, PathBuf};
use tar::Archive;
use xz2::read::XzDecoder;
use flate2::read::GzDecoder;

use zip_extract;
use crate::packages::ExtractMethodEnum;






pub fn extract_package(method: ExtractMethodEnum, downloaded_file_path :&PathBuf, extract_dir: &Path) -> Result<(), zip_extract::ZipError> {
use ExtractMethodEnum::*;

    // TODO!! Support more extentions
    let _ = match method {
        Zip => extract_zip(downloaded_file_path, extract_dir),
        Tarxz => extract_tar_xz(downloaded_file_path, extract_dir),
        // Git  => {},
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


fn extract_tar_gz(file_from: &Path, _file_to: &Path) -> Result<(), io::Error> {

    let tar_gz = File::open(file_from)?;
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    archive.unpack(".")?;
    Ok(())
}



fn extract_zip(file_to_extract : &Path, destination : &Path)-> io::Result<()>  {

    let file_to_extract = File::open(file_to_extract).unwrap();

    zip_extract::extract(file_to_extract, destination, true).expect("could not extract");
    Ok(())
}
