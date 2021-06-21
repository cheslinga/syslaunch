#![feature(cstring_from_vec_with_nul)]
mod calls;
mod sig;
mod helpers;

use crate::{calls::*, sig::*, helpers::*};
use nix::sys::signal::*;

fn main() {
    let act_int = construct_sigact(SIGINT);
    let act_usr1 = construct_sigact(SIGUSR1);
    let act_alrm = construct_sigact(SIGALRM);

    println!("BOOTSEQ OK");
    println!("Starting init...");

    mount_dirs();

    exec_process(
        &String::from("/bin/zsh").to_cstring_with_null(),
        &construct_array(&vec![]),
        &construct_array(&vec![])
    );

    loop { unsafe {
        sigaction(SIGUSR1, &act_usr1);
        sigaction(SIGINT, &act_int);
        sigaction(SIGALRM, &act_alrm);
    } }
}