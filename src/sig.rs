use nix::sys::signal::*;
use nix::libc::c_int;
use crate::calls::*;

pub fn construct_sigact(sig: Signal) -> SigAction {
    let fn_ptr = match sig {
        SIGINT => handle_sigint,
        SIGUSR1 => handle_sigusr1,
        SIGALRM => handle_alrm,
        _ => panic!("Unrecognized signal in handler construction!!")
    };
    let handler = SigHandler::Handler(fn_ptr);
    SigAction::new(handler, SaFlags::empty(), SigSet::empty())
}

extern fn handle_sigint(_sig: c_int) { p_shutdown() }
extern fn handle_sigusr1(_sig: c_int) { p_reboot() }
extern fn handle_alrm(_sig: c_int) { println!("\n!!TEST ALARM!!"); }
