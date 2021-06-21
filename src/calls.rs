use nix::sys::reboot::*;
use nix::unistd::*;
use std::ffi::CString;
use nix::mount::{mount, MsFlags};
use nix::sys::socket::MsgFlags;

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

pub fn exec_process(path: &CString, args: &Vec<CString>, env: &Vec<CString>) {
    match unsafe {fork()} {
        Err(e) => println!("|!!| Fork failed with fatal error {}", e),
        Ok(ForkResult::Parent { child, .. }) => println!("Init spawned child process ({})!", child),
        Ok(ForkResult::Child) => {
            execve(path.as_c_str(), &*args, &*env);
        }
    }
}

pub fn mount_dirs() {
    let mut errors = String::new();

    if let Some(_) = mount::<str, str, str, str>(Some("proc"), "/proc", Some("proc"),
                           MsFlags::MS_NOEXEC | MsFlags::MS_NOSUID | MsFlags::MS_NODEV, None).err() { errors.push_str("proc ") }
    if let Some(_) = mount::<str, str, str, str>(Some("sys"), "/sys", Some("sysfs"),
                           MsFlags::MS_NOEXEC | MsFlags::MS_NOSUID | MsFlags::MS_NODEV, None).err() { errors.push_str("sys ") }
    if let Some(_) = mount::<str, str, str, str>(Some("run"), "/run", Some("tmpfs"),
                           MsFlags::MS_NOSUID | MsFlags::MS_NODEV, Some("mode=0755")).err() { errors.push_str("run ") }
    if let Some(_) = mount::<str, str, str, str>(Some("dev"), "/dev", Some("devtmpfs"),
                           MsFlags::MS_NOSUID, Some("mode=0755")).err() { errors.push_str("dev ") }

    if !errors.is_empty() {
        eprintln!("|!!| Unable to mount the following filesystems:\n|!!| {}", errors);
    }
}