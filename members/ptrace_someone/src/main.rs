use std::path::{Path, PathBuf};
use std::process::{exit, Child, Command};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("USAGE: {} DEBUGEE [ARGS]", args[0]);
        exit(1)
    }
    let path_to_executable: PathBuf = match args[1].parse() {
        Err(e) => {
            eprintln!("error getting the path '{}': {e}", args[1]);
            exit(2);
        }
        Ok(p) => p,
    };
    let process_args: Vec<String> = args.iter().skip(2).map(|a| a.to_string()).collect();

    if !path_to_executable.is_file() {
        eprintln!("not a file '{}'", args[1]);
        exit(2);
    }

    let mut child =
        launch(path_to_executable, &process_args).expect("could not launch child process");

    child.wait().expect("child died :(");
}

fn launch(path_to_executable: impl AsRef<Path>, args: &[String]) -> Result<Child, std::io::Error> {
    let mut cmd = Command::new(path_to_executable.as_ref());
    for arg in args {
        cmd.arg(arg);
    }
    cmd.spawn()
}
