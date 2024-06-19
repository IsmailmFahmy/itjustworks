use std::path::PathBuf;
use std::fs::File;
use std::io::Cursor;
use std::io;
use reqwest::*;
use tempdir::TempDir;


#[tokio::main]
pub async fn download_files( link : &str, name :&str, path: &TempDir) -> Result<()> {

    let mut file_path = File::create(path.path().join(name)).unwrap();
    // create full path + Filename

    println!("started function...");

    let response = reqwest::get(link).await?;




    let mut content = Cursor::new(response.bytes().await?);

    io::copy(&mut content, &mut file_path).unwrap();
    println!("File downloaded successfully! at {:?}", file_path);

    Ok(())
}

