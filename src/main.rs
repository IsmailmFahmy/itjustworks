#![allow(dead_code)]
#![allow(unused)]
extern crate tempdir;
// extern crate reqwest;

use std::fs::File;
use std::path::PathBuf;
use std::io;
use tempdir::TempDir;


mod download;
use download::*;


// How should this be downloaded / Installed
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
    let libnotify = Package {   
        name: "libnotify",
        sourcecode_link: "https://download.gnome.org/sources/libnotify/0.8/libnotify-0.8.3.tar.xz",
        instal_method:  InstallMethodEnum::Targz
    };

    let selected = vec![libnotify];
    run_in_temp_dir(selected);



    // For testing download_files function (It doesn't work lol)

    // let mut file = File::create("/home/fahmy/Hello.tar.xz").unwrap();
    // download_files(libnotify.sourcecode_link, &mut file);


}







fn run_in_temp_dir(selected :Vec<Package>) -> Result<(), io::Error> {
    // Create a directory inside of `std::env::temp_dir()`, named with
    // the prefix "example".
    let mut tmp_dir = TempDir::new("ItJustWorks").expect("no again");
    let file_path = tmp_dir.path().join("my-temporary-note.tar.gz");


    for package in selected {

        // TODO : Find a way to turn the filepath (type PathBuf) into File
        download_files(&package.sourcecode_link, &mut file_path);
    }

    // been deleted successfully. If we don't close it explicitly,
    // the directory will still be deleted when `tmp_dir` goes out
    // of scope, but we won't know whether deleting the directory
    // succeeded.

    Ok(())
}

