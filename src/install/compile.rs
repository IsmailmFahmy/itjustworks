use crate::packages::InstallMethodEnum;
use crate::utils::cmd;


pub fn install_package(method: &InstallMethodEnum) -> Result<String, String> {
    use InstallMethodEnum::*;
    match method {
        MakeInstall => make_install()?,
        AutoGen => autogen_install()?
    };
    Ok(String::from("Installed successfully"))
}


fn autogen_install() -> Result<String, String>  {

    // Run ./autogen.sh ; ./configure && make
    // 
    let output = cmd("./autogen.sh; ./configure && make && sudo make install");
    output

}


fn make_install() -> Result<String, String> {

    // run make command
    let _ = cmd("make")?;

    // run sudo make install command
    let install_output = cmd("sudo make install")?;
    Ok(install_output)
}
