use std::process::exit;

use nix::sys::signal::{self, Signal};

fn main() {
    println!("Hello, world!");

    // send SIGTRAP
    // if no debugger is present, this will terminate the program with exit code 133, as there is no signal handler
    // present
    //
    // Jobs terminated with a system signal are returned by LSF as exit codes
    // greater than 128 such that exit_code-128=signal_value. For example, exit
    // code 133 means that the job was terminated with signal 5 (SIGTRAP on most
    // systems, 133-128=5). A job with exit code 130 was terminated with signal 2
    // (SIGINT on most systems, 130-128 = 2).
    if let Err(e) = signal::raise(Signal::SIGTRAP) {
        eprintln!("could not SIGTRAP: {e}");
        exit(1);
    }

    println!("sigtrap was sent");
    println!("exit normally")
}
