extern crate nix;

use std::ffi::CString;
use nix::sys::wait::*;
use nix::unistd::*;

fn main() {
    match fork().expect("fork(2) failed") {
        ForkResult::Parent { child } => {
            match waitpid(child, None).expect("wait_pid failed") {
                WaitStatus::Exited(pid, status) => {
                    println!("exit!: pid={:?}, status={:?}", pid, status)
                }
                WaitStatus::Signaled(pid, status, _) => {
                    println!("signal!: pid={:?}, status={:?}", pid, status)
                }
                _ => println!("abnormal exit!"),
            }
        }
        ForkResult::Child => {
            execv(
                &CString::new("/bin/echo").unwrap(),
                &[
                    CString::new("/bin/echo").unwrap(),
                    CString::new("OK").unwrap(),
                ],
            ).expect("execution failed.");
        }
    }
}
