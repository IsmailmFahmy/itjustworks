use crate::packages::InstallMethodEnum;
use std::process::Command;



pub fn install_package(method: &InstallMethodEnum) -> Result<String, String> {
    // todo!("Make an error type for Install package function");
    use InstallMethodEnum::*;
    match method {
        MakeInstall => make_install()?,
        AutoGen => autogen_install()?
    };
    Ok(String::from("Installed successfully"))
}


fn autogen_install() -> Result<String, String>  {

    // Run ./autogen.sh && ./configure && make
    let output = Command::new("sh")
        .arg("-c")
        .arg("./autogen.sh; ./configure && make && sudo make install")
        .output() // collect ouput
        .map_err(|e| format!("Failed to Execute shell command: \t {}", e))?; // format error


    if output.status.success() {
        // Turn the result into a readable string
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}


fn make_install() -> Result<String, String> {

    // run make command
    let make_output = Command::new("make")
        .output()
        .map_err(|e| format!("Failed to Execute shell command: \n{}\n", e))?;

    // run sudo make install command
    let install_output = Command::new("sudo")
        .arg("make")
        .arg("install")
        .output()
        .map_err(|e| format!("Failed to Execute shell command: \n{}\n", e))?;




    // Turn the errors into readable strings
    let make_err = String::from_utf8_lossy(&make_output.stderr).to_string();
    let install_err = String::from_utf8_lossy(&install_output.stderr).to_string();

    // Return combined errors if any
    if !make_output.status.success() || !install_output.status.success() {
        return Err(format!("{}\n{}", make_err, install_err));
    }
    Ok(String::from_utf8_lossy(&install_output.stdout).to_string())

}
