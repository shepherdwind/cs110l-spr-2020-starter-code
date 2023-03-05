use std::collections::HashMap;

use crate::debugger_command::DebuggerCommand;
use crate::dwarf_data::DwarfData;
use crate::inferior::{Inferior, Status};
use nix::sys::ptrace;
use rustyline::error::ReadlineError;
use rustyline::Editor;

pub struct Debugger {
    target: String,
    history_path: String,
    readline: Editor<()>,
    inferior: Option<Inferior>,
    dwarf_data: DwarfData,
    break_points: HashMap<usize, Breakpoint>,
}

#[derive(Clone)]
pub struct Breakpoint {
    pub addr: usize,
    pub orig_byte: u8,
}


fn parse_address(addr: &str) -> Option<usize> {
    let addr_without_0x = if addr.to_lowercase().starts_with("0x") {
        &addr[2..]
    } else {
        &addr
    };
    usize::from_str_radix(addr_without_0x, 16).ok()
}

impl Debugger {
    /// Initializes the debugger.
    pub fn new(target: &str) -> Debugger {
        let dwarf = DwarfData::from_file(target).unwrap();
        dwarf.print();

        let history_path = format!("{}/.deet_history", std::env::var("HOME").unwrap());
        let mut readline = Editor::<()>::new();
        // Attempt to load history from ~/.deet_history if it exists
        let _ = readline.load_history(&history_path);

        Debugger {
            target: target.to_string(),
            history_path,
            readline,
            inferior: None,
            dwarf_data: dwarf,
            break_points: HashMap::new(),
        }
    }

    pub fn run(&mut self) {
        loop {
            match self.get_next_command() {
                DebuggerCommand::Run(args) => self.run_command(args),
                DebuggerCommand::Quit => {
                    return;
                }
                DebuggerCommand::Cont => {
                    if let Err(err) = self.cont_command() {
                        println!("continue command fail {}", err);
                    }
                },
                DebuggerCommand::BackTrace => self.back_trace(),
                DebuggerCommand::BreakPoint(pos) => self.break_point_command(pos),
            }
        }
    }

    fn run_command(&mut self, args: Vec<String>) {
        if let Some(infer) = self.inferior.as_mut() {
            infer.kill();
        }

        if let Some(inferior) = Inferior::new(&self.target, &args, &mut self.break_points) {
            self.inferior = Some(inferior);
            self.cont_run();
        } else {
            println!("Error starting subprocess");
        }
    }

    fn cont_command(&mut self) -> Result<(), nix::Error> {
        if self.inferior.is_none() {
            println!("no process in debugger");
            return Ok(());
        }
        let inferior = self.inferior.as_mut().unwrap();
        let pid = inferior.pid();
        let mut registers = ptrace::getregs(pid)?;
        let rip = registers.rip - 1;
        let break_pointer = self.break_points.get(&(rip as usize));

        if break_pointer.is_none() {
            return Ok(());
        }

        let address = break_pointer.unwrap().addr;
        let orig_byte = break_pointer.unwrap().orig_byte;

        inferior.write_byte(address, orig_byte)?;

        registers.rip = rip;
        ptrace::setregs(pid, registers)?;

        // step forward
        ptrace::step(pid, None)?;
        let status = inferior.wait(None)?;
        if let Status::Exited(_) = status {
            self.inferior = None;
            return Ok(());
        }

        // wite back  debug command
        inferior.write_byte(address, 0xcc)?;
        self.cont_run();

        Ok(())
    }

    fn break_point_command(&mut self, position: Option<String>) {
        if position.is_none() {
            println!("please input position");
            return;
        }
        let str = position.unwrap();
        if let Some(addr) = parse_address(&str) {
            self.break_points.insert(addr, Breakpoint { addr, orig_byte: 0 });
            println!(
                "Set breakpoint {} at {}",
                self.break_points.len() - 1,
                &str[1..]
            );
        } else {
            println!("parse address: {} fail", str);
        }
    }

    fn back_trace(&mut self) {
        if self.inferior.is_none() {
            println!("No process is running");
            return;
        }
        let pid = self.inferior.as_ref().unwrap().pid();
        let registers = ptrace::getregs(pid).unwrap();
        let mut instruction_ptr = registers.rip as usize;
        let mut base_ptr = registers.rbp as usize;
        loop {
            let path = self.dwarf_data.get_line_from_addr(instruction_ptr).unwrap();
            let func = self
                .dwarf_data
                .get_function_from_addr(instruction_ptr)
                .unwrap();
            println!("{} ({}:{})", func, path.file, path.number);
            if func == "main" {
                break;
            }
            instruction_ptr =
                ptrace::read(pid, (base_ptr + 8) as ptrace::AddressType).unwrap() as usize;
            base_ptr = ptrace::read(pid, base_ptr as ptrace::AddressType).unwrap() as usize;
        }
    }

    fn cont_run(&mut self) {
        if self.inferior.is_none() {
            println!("No process is running");
            return;
        }
        ptrace::cont(self.inferior.as_mut().unwrap().pid(), None).ok();
        match self.wait() {
            Ok(ret) => println!("Child {}", ret),
            Err(err) => println!("wait fail {}", err),
        };
    }

    fn wait(&mut self) -> Result<String, nix::Error> {
        let status = self.inferior.as_mut().unwrap().wait(None)?;

        let ret = match status {
            Status::Stopped(sig, rip) => {
                let path = self.dwarf_data.get_line_from_addr(rip).unwrap();
                format!(
                    "Stopped (status {})\nStopped at {}:{}",
                    sig.as_str(),
                    path.file,
                    path.number
                )
            }
            Status::Exited(code) => {
                self.inferior = None;
                format!("Exited (Status {})", code)
            }
            Status::Signaled(n) => format!("Signal (Status {})", n.as_str()),
        };
        Ok(ret)
    }

    /// This function prompts the user to enter a command, and continues re-prompting until the user
    /// enters a valid command. It uses DebuggerCommand::from_tokens to do the command parsing.
    ///
    /// You don't need to read, understand, or modify this function.
    fn get_next_command(&mut self) -> DebuggerCommand {
        loop {
            // Print prompt and get next line of user input
            match self.readline.readline("(deet) ") {
                Err(ReadlineError::Interrupted) => {
                    // User pressed ctrl+c. We're going to ignore it
                    println!("Type \"quit\" to exit");
                }
                Err(ReadlineError::Eof) => {
                    // User pressed ctrl+d, which is the equivalent of "quit" for our purposes
                    return DebuggerCommand::Quit;
                }
                Err(err) => {
                    panic!("Unexpected I/O error: {:?}", err);
                }
                Ok(line) => {
                    if line.trim().len() == 0 {
                        continue;
                    }
                    self.readline.add_history_entry(line.as_str());
                    if let Err(err) = self.readline.save_history(&self.history_path) {
                        println!(
                            "Warning: failed to save history file at {}: {}",
                            self.history_path, err
                        );
                    }
                    let tokens: Vec<&str> = line.split_whitespace().collect();
                    if let Some(cmd) = DebuggerCommand::from_tokens(&tokens) {
                        return cmd;
                    } else {
                        println!("Unrecognized command.");
                    }
                }
            }
        }
    }
}
