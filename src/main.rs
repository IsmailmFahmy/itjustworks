#![allow(dead_code)]
#![allow(unused)]

use tempdir::TempDir;

mod error_handelling;

use std::fs::File;
use std::path::{Path, PathBuf};
use std::io;
use std::env;

pub mod download;
use download::*;

mod extract;
use extract::*;

mod checks;

mod packages;
use packages::*;

mod install;
use crate::install::compile::install_package;

fn main() -> Result<(), io::Error> {

    // Testing with libnotify package
    let libnotify = Package {   
        name: "libnotify",
        sourcecode_link: "https://download.gnome.org/sources/libnotify/0.8/libnotify-0.8.3.tar.xz",
        extract_method:  ExtractMethodEnum::Tarxz,
        instal_method : InstallMethodEnum::MakeInstall

    };


    let htop = Package {
        name: "htop",
        sourcecode_link: "https://github.com/htop-dev/htop/archive/refs/heads/main.zip",
        extract_method: ExtractMethodEnum::Zip,
        instal_method: InstallMethodEnum::AutoGen
    };

    let tmux = Package {
        name: "tmux",
        sourcecode_link: "https://github.com/tmux/tmux/releases/download/3.4/tmux-3.4.tar.gz",
        extract_method: ExtractMethodEnum::Targz,
        instal_method: InstallMethodEnum::AutoGen
    };


    let bluetooth = Package {
        name : "bluez",
        sourcecode_link : "http://www.kernel.org/pub/linux/bluetooth/bluez-5.66.tar.xz",
        extract_method : ExtractMethodEnum::Tarxz,
        instal_method: InstallMethodEnum::AutoGen
    };
    // todo!("select_packages");

    let selected = vec![libnotify];





    // Create a directory inside of `std::env::temp_dir()`, named with
    // the prefix "ItJustWorks".
    let mut tmp_dir = TempDir::new("ItJustWorks").expect("could not create temporary directory");


    for package in selected {

        let sys = checks::get_sysinfo();
        println!("{:?}", sys);
        // download each package                         link                 path
        let downloaded_file_path = download_files(package.sourcecode_link , &tmp_dir)
            .expect(format!("Failed to download the {} package", package.name).as_str());


        let working_path = tmp_dir.path();

        env::set_current_dir(&working_path);

        extract_package(package.extract_method, &downloaded_file_path, &working_path.join(package.name))
            .expect(format!("Failed to extract the {} package", package.name).as_str());


        println!("extracted package");

        //change directory to the extracted package
        env::set_current_dir(&working_path.join(package.name));


        pause();
        install_package(package.instal_method);
        println!("package installed");

        
    }




    Ok(())
}









// read stdin to pause execution for debugging
fn pause() {
    println!("Pausing... Please press ENTER to continue excecution");
    let mut buffer = String::new();
    std::io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line");

}
