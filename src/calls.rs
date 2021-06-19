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

pub fn exec_process(path: &CStr, args: &Vec<&CStr>, env: &Vec<&CStr>) {
    match unsafe {fork()} {
        Err(e) => println!("|!!| Fork failed with fatal error {}", e),
        Ok(ForkResult::Parent { child, .. }) => println!("Init spawned child process ({})!", child),
        Ok(ForkResult::Child) => { execve(path, &*args, &*env); }
    }
}