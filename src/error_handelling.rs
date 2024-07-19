// Matches and prints errors
pub fn handle(result: Result<String, String>, print_if_success: bool) {

    match result {
        Ok(output) => {
            if print_if_success {
            println!("Command output: {}", output)
            }
        },
        Err(error) => panic!("Error running command: \n{}\n", error),
    }
}
