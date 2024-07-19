// #![allow(dead_code)]
// #![allow(unused)]

use tempdir::TempDir;


use std::io;
use std::env;

use itjustworks::download::*;
use itjustworks::extract::*;
use itjustworks::packages::*;



use itjustworks::install::compile::install_package;
use itjustworks::install::enable_service::enable_services;
use itjustworks::checks::SysInfo;

fn main() -> Result<(), io::Error> {

    // Testing with libnotify package
    let libnotify = Package {   
        name: "libnotify",
        sourcecode_link: "https://download.gnome.org/sources/libnotify/0.8/libnotify-0.8.3.tar.xz",
        extract_method:  ExtractMethodEnum::Tarxz,
        instal_method : InstallMethodEnum::MakeInstall,
        service: None,

    };


    let cups = Package {
        name : "cups",
        sourcecode_link: "https://github.com/OpenPrinting/cups/archive/refs/heads/master.zip",
        extract_method: ExtractMethodEnum::Zip,
        instal_method: InstallMethodEnum::AutoGen,
        service: Some("cups.service")
    };

    let htop = Package {
        name: "htop",
        sourcecode_link: "https://github.com/htop-dev/htop/archive/refs/heads/main.zip",
        extract_method: ExtractMethodEnum::Zip,
        instal_method: InstallMethodEnum::AutoGen,
        service: None
        
    };

    let tmux = Package {
        name: "tmux",
        sourcecode_link: "https://github.com/tmux/tmux/releases/download/3.4/tmux-3.4.tar.gz",
        extract_method: ExtractMethodEnum::Targz,
        instal_method: InstallMethodEnum::AutoGen,
        service: None

    };


    let bluetooth = Package {
        name : "bluez",
        sourcecode_link : "http://www.kernel.org/pub/linux/bluetooth/bluez-5.66.tar.xz",
        extract_method : ExtractMethodEnum::Tarxz,
        instal_method: InstallMethodEnum::AutoGen,
        service: Some("bluetooth.service")


    };
    // todo!("select_packages");

    let selected = vec![cups];



    // Create a directory inside of `std::env::temp_dir()`, named with
    // the prefix "ItJustWorks".
    let tmp_dir = TempDir::new("ItJustWorks").expect("could not create temporary directory");

    let sysinfo = SysInfo::new(); // Defining system

    for package in selected {



        // download each package                         link                 path
        let downloaded_file_path = download_files(package.sourcecode_link , &tmp_dir)
            .expect(format!("Failed to download the {} package", package.name).as_str());


        let working_path = tmp_dir.path();

        let _ = env::set_current_dir(&working_path);

        extract_package(&package.extract_method, &downloaded_file_path, &working_path.join(package.name))
            .expect(format!("Failed to extract the {} package", package.name).as_str());


        println!("extracted package");

        //change directory to the extracted package
        let _ = env::set_current_dir(&working_path.join(package.name));


        // pause();
        install_package(&package.instal_method);
        println!("package installed");

        println!("Enabling services where needed");

        enable_services(&package, &sysinfo);



        
    };




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
