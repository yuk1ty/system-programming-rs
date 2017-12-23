extern crate nix;

use std::env;
use std::ffi::CString;
use nix::sys::wait::*;
use nix::unistd::*;

fn main() {
    match fork().expect("fork failed") {
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
            let args: Vec<String> = env::args().collect();
            let dir = CString::new(args[1].to_string()).unwrap();
            let arg = CString::new(args[2].to_string()).unwrap();

            execv(
                &dir,
                &[
                    dir.clone(),
                    arg,
                ],
            ).expect("execution failed.");
        }
    }
}
