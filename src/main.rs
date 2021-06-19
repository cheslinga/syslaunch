mod calls;
mod sig;

use crate::{calls::*, sig::*};
use nix::sys::signal::*;
use std::ffi::CStr;

fn main() {
    let act_int = construct_sigact(SIGINT);
    let act_usr1 = construct_sigact(SIGUSR1);
    let act_alrm = construct_sigact(SIGALRM);

    println!("BOOTSEQ OK");
    println!("Starting init...");

    exec_process(
        CStr::from_bytes_with_nul("/bin/zsh\0".as_bytes()).unwrap(),
        &vec![CStr::from_bytes_with_nul("\0".as_bytes()).unwrap()],
        &vec![CStr::from_bytes_with_nul("\0".as_bytes()).unwrap()]
    );

    loop { unsafe {
        sigaction(SIGUSR1, &act_usr1);
        sigaction(SIGINT, &act_int);
        sigaction(SIGALRM, &act_alrm);
    } }
}