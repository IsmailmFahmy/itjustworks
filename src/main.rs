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



enum InstallMethodEnum {
    Git,
    Targz
}


struct Package<'a> {
    name: &'a str,
    sourcecode_link: &'a str,
    instal_method : InstallMethodEnum
}



fn main() {
    // Testing with libnotify package
    // let libnotify = Package {   
    //     name: "libnotify",
    //     sourcecode_link: "https://download.gnome.org/sources/libnotify/0.8/libnotify-0.8.3.tar.xz",
    //     // sourcecode_link : "https://github.com/IsmailmFahmy/nvim/archive/refs/heads/main.zip",
    //     instal_method:  InstallMethodEnum::Targz
    // };
    //
    // // List of packages to install
    // let selected = vec![libnotify];
    //
    // // Open Temp Directory
    // run_in_temp_dir(selected);


    checks::testie();
    checks::check_dependencies()


    // For testing download_files function (It doesn't work lol)

    // let mut file = File::create("/home/fahmy/Hello.tar.xz").unwrap();
    // download_files(libnotify.sourcecode_link, &mut file);


}








// read stdin to pause execution for testing
fn pause() {
    let mut buffer = String::new();
    std::io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line");

}




fn run_in_temp_dir(selected :Vec<Package>) -> Result<(), io::Error> {
    // Create a directory inside of `std::env::temp_dir()`, named with
    // the prefix "ItJustWorks".
    let mut tmp_dir = TempDir::new("ItJustWorks").expect("could not create temporary directory");



    for package in selected {
        // download each package
        let downloaded_file_path : PathBuf = download_files(package.sourcecode_link , &tmp_dir).unwrap();
        //                                                         link                 path



        println!("after downloading, the downloaded file path is : {:?}",downloaded_file_path);
        // pause();

        
        let extract_dir = tmp_dir.path();

        extract_tar_xz(&downloaded_file_path, &extract_dir);

        println!("extracting file in {:?}", extract_dir);

        // pause();

    }



    Ok(())
}






