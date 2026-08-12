use std::process::ExitCode;

fn main() -> ExitCode {
    match grey_owl::cli::run(std::env::args().skip(1).collect()) {
        Ok(code) => ExitCode::from(code),
        Err(message) => {
            eprintln!("growl: {message}");
            ExitCode::from(2)
        }
    }
}
