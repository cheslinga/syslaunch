use nix::sys::reboot::*;
use nix::unistd::*;
use std::ffi::CString;
use nix::mount::{mount, MsFlags};

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

    if let Some(e) = mount::<str, str, str, str>(Some("proc"), "/proc", Some("proc"),
                           MsFlags::MS_NOEXEC | MsFlags::MS_NOSUID | MsFlags::MS_NODEV, None).err() { eprintln!("|!!| Error mounting proc: {}", e) }
    if let Some(e) = mount::<str, str, str, str>(Some("sys"), "/sys", Some("sysfs"),
                           MsFlags::MS_NOEXEC | MsFlags::MS_NOSUID | MsFlags::MS_NODEV, None).err() { eprintln!("|!!| Error mounting sysfs: {}", e) }
    if let Some(e) = mount::<str, str, str, str>(Some("run"), "/run", Some("tmpfs"),
                           MsFlags::MS_NOSUID | MsFlags::MS_NODEV, Some("mode=0755")).err() { eprintln!("|!!| Error mounting tmpfs: {}", e) }
    if let Some(e) = mount::<str, str, str, str>(Some("dev"), "/dev", Some("devtmpfs"),
                           MsFlags::MS_NOSUID, Some("mode=0755")).err() { eprintln!("|!!| Error mounting devtmpfs: {}", e) }
}