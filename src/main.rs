#![allow(dead_code)]
#![allow(unused)]

use std::fs::File;
use std::path::{Path, PathBuf};
use std::io;
use tempdir::TempDir;
use zip_extract;


mod download;
use download::*;

mod extract;
use extract::*;

mod checks;

mod packages;
use packages::*;



fn main() -> Result<(), io::Error> {

    // Testing with libnotify package
    let libnotify = Package {   
        name: "libnotify",
        sourcecode_link: "https://download.gnome.org/sources/libnotify/0.8/libnotify-0.8.3.tar.xz",
        instal_method:  InstallMethodEnum::Targz
    };



    todo!("select_packages");

    let selected = vec![libnotify];





    // Create a directory inside of `std::env::temp_dir()`, named with
    // the prefix "ItJustWorks".
    let mut tmp_dir = TempDir::new("ItJustWorks").expect("could not create temporary directory");



    // List of packages to install

    for package in selected {
        // download each package                                   link                 path
        let downloaded_file_path : PathBuf = download_files(package.sourcecode_link , &tmp_dir).unwrap();


        println!("after downloading, the downloaded file path is : {:?}",downloaded_file_path);

        let extract_dir = tmp_dir.path();

        match package.instal_method {
            InstallMethodEnum::Targz => extract_tar_xz(&downloaded_file_path, &extract_dir),
            InstallMethodEnum::Git  => todo!("do!"),
        };
        

        println!("extracting file in {:?}", extract_dir);


    }



    Ok(())
}




// read stdin to pause execution for testing
fn pause() {
    let mut buffer = String::new();
    std::io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line");

}


