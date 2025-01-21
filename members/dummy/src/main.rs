use std::thread::sleep;
use std::time::Duration;

use nix::libc;
use nix::sys::signal;

static mut STATUS: Option<libc::c_int> = None;

// does nothing and wastes time
fn main() {
    install_handler();
    loop {
        unsafe {
            if let Some(sig) = STATUS {
                println!("(dummy): got sig: {sig}");
                STATUS = None;
            }
        }
        sleep(Duration::from_millis(50))
    }
}

fn install_handler() {
    let sigs = [
        signal::SIGTRAP,
        signal::SIGTERM,
        signal::SIGCONT,
        signal::SIGHUP,
        signal::SIGINT,
    ];
    unsafe {
        let sig_action = signal::SigAction::new(
            signal::SigHandler::SigAction(handle),
            signal::SaFlags::empty(),
            signal::SigSet::empty(),
        );
        for s in sigs {
            signal::sigaction(s, &sig_action).expect("could not register handler for signal");
        }
    }
}

extern "C" fn handle(sig: libc::c_int, siginfo: *mut libc::siginfo_t, idk: *mut libc::c_void) {
    unsafe { STATUS = Some(sig) }
}
