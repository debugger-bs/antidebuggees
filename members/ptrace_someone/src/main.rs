use std::path::{Path, PathBuf};
use std::process::{exit, Child, Command};

use nix::libc::WUNTRACED;
use nix::sys::ptrace::{self, Options};
use nix::sys::wait::{waitpid, WaitPidFlag};
use nix::unistd::Pid;

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

    trace(&child);
    // child.wait().expect("child is dead");

    print!("DONE");
}

fn trace(child: &Child) {
    let pid = Pid::from_raw(child.id() as i32);
    println!("pid of child: {pid}");
    // From man ptrace
    // PTRACE_ATTACH
    //   Attach  to  the  process specified in pid, making it a tracee of the calling process.
    //   The tracee is sent a SIGSTOP, but will not
    //   necessarily have stopped by the completion of this call; use waitpid(2) to wait for the tracee to stop.
    //   See the "Attaching and detaching" // subsection for
    //   additional information.  (addr and data are ignored.)
    ptrace::attach(pid).expect("could not attach ptrace to child");
    // wait until the child processes the SIGSTOP
    match waitpid(pid, Some(WaitPidFlag::WUNTRACED)) {
        Ok(ws) => println!("waited for {pid} to change status success: it is now {ws:?}"),
        Err(e) => eprintln!("waited for {pid} to change status failure: {e}"),
    }
    // from man ptrace:
    // ESRCH  The specified process does not exist, or is not currently being traced by the caller, or is not stopped (for requests that require a stopped tracee).
    if let Err(e) = ptrace::detach(pid, None) {
        eprintln!("ptrace cannot detach: {e}");
        exit(4)
    }
}

fn launch(path_to_executable: impl AsRef<Path>, args: &[String]) -> Result<Child, std::io::Error> {
    let mut cmd = Command::new(path_to_executable.as_ref());
    for arg in args {
        cmd.arg(arg);
    }
    cmd.spawn()
}
