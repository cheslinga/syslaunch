mod calls;
mod sig;

use crate::{calls::*, sig::*};
use nix::unistd::getpid;
use nix::sys::signal::*;

fn main() {
    let act_int = construct_sigact(SIGINT);
    let act_usr1 = construct_sigact(SIGUSR1);
    let act_alrm = construct_sigact(SIGALRM);

    println!("BOOTSEQ OK");
    println!("Starting init...");
    println!("({}) Finished init", getpid().as_raw());

    exec_process(b"/bin/zsh\0");

    loop { unsafe {
        sigaction(SIGUSR1, &act_usr1);
        sigaction(SIGINT, &act_int);
        sigaction(SIGALRM, &act_alrm);
    } }
}