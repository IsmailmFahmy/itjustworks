use std::path::PathBuf;
use std::fs::File;
use std::io::Cursor;
use std::io;
use reqwest::*;




pub async fn download_files(link: &str, name: &mut File) -> Result<()> {



    println!("started function...");
    let response = reqwest::get(link).await?;

    println!("response");
    // let file = File::create("downloaded_file").unwrap();
    let mut content = Cursor::new(response.bytes().await?);
    io::copy(&mut content, name).unwrap();
    println!("File downloaded successfully!");

    Ok(())
}

