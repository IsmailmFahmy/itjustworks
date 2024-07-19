use std::process::{Output,Command};

pub fn cmd(command: &str) -> Result<String, String> {
    let output = Command::new("sh")
        .arg("-c")
        .arg(command)
        .output();

    match output {
        Ok(Output { status, stdout, stderr }) => {
            if status.success() {
                Ok(String::from_utf8_lossy(&stdout).into_owned())
            } else {
                Err(String::from_utf8_lossy(&stderr).into_owned())
            }
        }
        Err(e) => Err(e.to_string()),
    }
}
