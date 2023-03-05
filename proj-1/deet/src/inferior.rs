use nix::sys::ptrace;
use nix::sys::signal;
use nix::sys::wait::{waitpid, WaitPidFlag, WaitStatus};
use nix::unistd::Pid;
use std::collections::HashMap;
use std::mem::size_of;
use std::process::Child;
use std::os::unix::process::CommandExt;
use std::process::Command;

use crate::debugger::Breakpoint;

pub enum Status {
    /// Indicates inferior stopped. Contains the signal that stopped the process, as well as the
    /// current instruction pointer that it is stopped at.
    Stopped(signal::Signal, usize),

    /// Indicates inferior exited normally. Contains the exit status code.
    Exited(i32),

    /// Indicates the inferior exited due to a signal. Contains the signal that killed the
    /// process.
    Signaled(signal::Signal),
}

/// This function calls ptrace with PTRACE_TRACEME to enable debugging on a process. You should use
/// pre_exec with Command to call this in the child process.
fn child_traceme() -> Result<(), std::io::Error> {
    ptrace::traceme().or(Err(std::io::Error::new(
        std::io::ErrorKind::Other,
        "ptrace TRACEME failed",
    )))
}

pub struct Inferior {
    child: Child,
}

fn align_addr_to_word(addr: usize) -> usize {
    addr & (-(size_of::<usize>() as isize) as usize)
}

impl Inferior {
    /// Attempts to start a new inferior process. Returns Some(Inferior) if successful, or None if
    /// an error is encountered.
    pub fn new(target: &str, args: &Vec<String>, break_pointers: &mut HashMap<usize, Breakpoint>) -> Option<Inferior> {
        let mut command = Command::new(target);
        unsafe {
            command.pre_exec::<_>(child_traceme);
        }
        println!(
            "Inferior run! target={}, args={:?}",
            target, args
        );
        let child = command.args(args).spawn().ok();

        match child {
            Some(c) => {
                let mut infer = Inferior { child: c };
                let wait = Inferior::wait(&infer, None).ok();
                if wait.is_none() {
                    return None;
                }

                for break_pointer in break_pointers.values_mut() {
                    match infer.write_byte(break_pointer.addr, 0xcc) {
                        Ok(orig_byte) => break_pointer.orig_byte = orig_byte,
                        Err(e) => println!("error happen: {}", e),
                    }
                }

                if let Status::Stopped(sig, _) = wait.unwrap() {
                    if sig == signal::Signal::SIGTRAP {
                        return Some(infer);
                    }
                }
                None
            },
            None => None
        }
    }

    /// Returns the pid of this inferior.
    pub fn pid(&self) -> Pid {
        nix::unistd::Pid::from_raw(self.child.id() as i32)
    }

    pub fn kill(&mut self) -> () {
        let pid = self.pid().to_string();
        let ret = self.child.kill();
        match ret {
            Ok(_) => println!("Killing running inferior (pid {})", pid),
            Err(e) => println!("Kill fail: {:?}", e),
        }
    }
    /// Calls waitpid on this inferior and returns a Status to indicate the state of the process
    /// after the waitpid call.
    pub fn wait(&self, options: Option<WaitPidFlag>) -> Result<Status, nix::Error> {
        Ok(match waitpid(self.pid(), options)? {
            WaitStatus::Exited(_pid, exit_code) => Status::Exited(exit_code),
            WaitStatus::Signaled(_pid, signal, _core_dumped) => Status::Signaled(signal),
            WaitStatus::Stopped(_pid, signal) => {
                let regs = ptrace::getregs(self.pid())?;
                Status::Stopped(signal, regs.rip as usize)
            }
            other => panic!("waitpid returned unexpected status: {:?}", other),
        })
    }

    pub fn write_byte(&mut self, addr: usize, val: u8) -> Result<u8, nix::Error> {
        let aligned_addr = align_addr_to_word(addr);
        let byte_offset = addr - aligned_addr;
        let word = ptrace::read(self.pid(), aligned_addr as ptrace::AddressType)? as u64;
        let orig_byte = (word >> 8 * byte_offset) & 0xff;
        let masked_word = word & !(0xff << 8 * byte_offset);
        let updated_word = masked_word | ((val as u64) << 8 * byte_offset);
        ptrace::write(
            self.pid(),
            aligned_addr as ptrace::AddressType,
            updated_word as *mut std::ffi::c_void,
        )?;
        Ok(orig_byte as u8)
    }
}
