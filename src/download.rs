use std::path::{PathBuf, Path};
use std::fs::File;
use std::io::Cursor;
use std::io;
use reqwest::*;
use tempdir::TempDir;


#[tokio::main]
pub async fn download_files( link : &str, path: &TempDir) -> Result<PathBuf> {

    // create full path + Filename

    println!("started function...");

    let response = reqwest::get(link).await?;

    let file_name = response
        .url()
        .path_segments()
        .and_then(|segments| segments.last())
        .and_then(|name| if name.is_empty() { None } else { Some(name) })
        .unwrap_or("tmp.bin");

    println!("file to download: '{}'", file_name);
    let file_name = path.path().join(file_name);
    println!("will be located under: '{:?}'", file_name);


    let mut file_path = File::create(&file_name).expect("could not create file");



    let mut content = Cursor::new(response.bytes().await?);

    io::copy(&mut content, &mut file_path).unwrap();
    println!("File downloaded successfully! at {:?}", file_path);

    Ok(file_name)
}

