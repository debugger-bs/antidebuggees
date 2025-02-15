use std::process::exit;

use nix::libc;
use nix::sys::signal::{self, Signal};

static mut WAS_HANDLED: bool = false;

fn main() {
    install_handler();
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

    if unsafe { WAS_HANDLED } {
        println!("no debugger attached")
    } else {
        println!("DEBUGGER IS ATTACHED; VERY EVIL");
        exit(1)
    }

    println!("exit normally")
}

fn install_handler() {
    unsafe {
        let sig_action = signal::SigAction::new(
            signal::SigHandler::SigAction(handle),
            signal::SaFlags::empty(),
            signal::SigSet::empty(),
        );
        signal::sigaction(signal::SIGTRAP, &sig_action)
    }
    .unwrap();
}

extern "C" fn handle(sig: libc::c_int, siginfo: *mut libc::siginfo_t, idk: *mut libc::c_void) {
    unsafe {
        WAS_HANDLED = true;
    }
}
