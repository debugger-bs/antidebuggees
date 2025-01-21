use std::path::{Path, PathBuf};
use std::process::{exit, Child, Command};
use std::thread::sleep_ms;

use nix::libc::{WUNTRACED, __WALL};
use nix::sys::ptrace::{self, Options};
use nix::sys::signal::Signal;
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

    child.kill().expect("cant kill child :(");
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
    //
    // Attach sucks because it cant do interrupt, that's seize only. Seize does not stop the process automatically too.
    ptrace::seize(pid, Options::empty()).expect("could not attach ptrace to child");
    analyze(pid);
    // from man ptrace:
    // ESRCH  The specified process does not exist, or is not currently being traced by the caller, or is not stopped (for requests that require a stopped tracee).
    if let Err(e) = ptrace::detach(pid, None) {
        eprintln!("ptrace cannot detach: {e}");
        exit(4)
    }
}

fn wait_status(pid: Pid) {
    // to wait until that child stops: None
    // to wait until that child continues: ???????
    match waitpid(pid, None) {
        Ok(ws) => println!("waited for {pid} to change status success: it is now {ws:?}"),
        Err(e) => eprintln!("waited for {pid} to change status failure: {e}"),
    }
}

fn analyze(pid: Pid) {
    //  PTRACE_INTERRUPT only works on tracees attached by PTRACE_SEIZE.
    println!("interrupting...");
    ptrace::interrupt(pid).unwrap();
    wait_status(pid);
    let mut regs = ptrace::getregs(pid).expect("could not get regs");
    println!("All Regs:\n{:#?}", regs);
    println!("RIP: {}", regs.rip);
    println!("step...");
    ptrace::step(pid, None).expect("could not step");
    wait_status(pid);
    regs = ptrace::getregs(pid).expect("could not get regs");
    println!("RIP: {}", regs.rip);
    println!("continuing...");
    ptrace::cont(pid, None).expect("could not continue child");
    // no need to wait because reasons
    sleep_ms(1000);
    println!("interrupting...");
    ptrace::interrupt(pid).unwrap();
    wait_status(pid);
}

fn launch(path_to_executable: impl AsRef<Path>, args: &[String]) -> Result<Child, std::io::Error> {
    let mut cmd = Command::new(path_to_executable.as_ref());
    for arg in args {
        cmd.arg(arg);
    }
    cmd.spawn()
}
