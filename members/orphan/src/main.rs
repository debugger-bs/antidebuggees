// The child does by default not get sent a signal if it's parent dies. To be notified of the parents death, we need to set pdeathsig or PR_SET_PDEATHSIG.
// With pdeathsig, the child can ask the kernel to send a signal of a particular type if it's parent dies.
// By convention, that signal seems to be SIGHUP
//
// Programs can hide from debuggers by forking and executing the code that is to be hidden in the child. This can even be combined
// with sending a SIGTRAP to check if a debugger catches it.
use std::process::exit;

use nix::{
    libc,
    sys::signal::{self, Signal},
    unistd::{fork, ForkResult},
};

fn main() {
    println!("hello world");
    // fork our process
    match unsafe { fork() } {
        Ok(ForkResult::Parent { child, .. }) => {
            install_handler(handle_p, Signal::SIGHUP);
            println!(
                "Continuing execution in parent process, new child has pid: {}",
                child
            );
            println!("(p) parent exit");
            std::thread::sleep_ms(300);
            exit(1)
        }
        Ok(ForkResult::Child) => {
            // we can request any signal from the kernel here: SIGTERM, SIGHUP, SIGKILL, SIGTRAP...
            nix::sys::prctl::set_pdeathsig(Some(Signal::SIGHUP)).expect("could not set pdeathsig");
            install_handler(handle_c, Signal::SIGHUP);
            println!("(c) I'M BORN");
            std::thread::sleep_ms(3000);
            println!("(c) I'M DYING OH NO");
        }
        Err(_) => println!("Fork failed"),
    }
}

fn install_handler(
    handler: extern "C" fn(i32, *mut libc::siginfo_t, *mut libc::c_void),
    sig: signal::Signal,
) {
    unsafe {
        let sig_action = signal::SigAction::new(
            signal::SigHandler::SigAction(handler),
            signal::SaFlags::empty(),
            signal::SigSet::empty(),
        );
        signal::sigaction(sig, &sig_action)
    }
    .unwrap();
}

// SIGHUP is 1, SIGTERM is 15...
extern "C" fn handle_c(sig: libc::c_int, siginfo: *mut libc::siginfo_t, idk: *mut libc::c_void) {
    println!("(c) got {sig:?}");
}

extern "C" fn handle_p(sig: libc::c_int, siginfo: *mut libc::siginfo_t, idk: *mut libc::c_void) {
    println!("(p) got {sig:?}");
}
