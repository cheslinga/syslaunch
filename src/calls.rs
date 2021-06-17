use nix::sys::reboot::*;
use nix::unistd::*;
use std::ffi::CStr;

pub fn p_shutdown() {
    println!("Init received signal for shutdown...");
    sync();
    reboot(RebootMode::RB_POWER_OFF);
}
pub fn p_reboot() {
    println!("Init received signal for reboot...");
    sync();
    reboot(RebootMode::RB_AUTOBOOT);
}

pub fn exec_process(filename: &[u8]) {
    let null = CStr::from_bytes_with_nul(&[0]).unwrap();

    match unsafe {fork()} {
        Err(e) => println!("|!!| Fork failed with fatal error {}", e),

        Ok(ForkResult::Parent { child, .. }) => println!("Init spawned child process ({})!", child),

        Ok(ForkResult::Child) => {
            let prog = CStr::from_bytes_with_nul(filename).unwrap();
            execve(prog,
                   &[null],
                   &[null]
            );
        }
    }
}
