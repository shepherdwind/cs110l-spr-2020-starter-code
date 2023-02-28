use crate::debugger_command::DebuggerCommand;
use crate::dwarf_data::DwarfData;
use crate::inferior::Inferior;
use nix::sys::ptrace;
use rustyline::error::ReadlineError;
use rustyline::Editor;

pub struct Debugger {
    target: String,
    history_path: String,
    readline: Editor<()>,
    inferior: Option<Inferior>,
    dwarfData: DwarfData,
}

impl Debugger {
    /// Initializes the debugger.
    pub fn new(target: &str) -> Debugger {
        // TODO (milestone 3): initialize the DwarfData

        let dwarf = DwarfData::from_file(target).unwrap();

        let history_path = format!("{}/.deet_history", std::env::var("HOME").unwrap());
        let mut readline = Editor::<()>::new();
        // Attempt to load history from ~/.deet_history if it exists
        let _ = readline.load_history(&history_path);

        Debugger {
            target: target.to_string(),
            history_path,
            readline,
            inferior: None,
            dwarfData: dwarf,
        }
    }

    pub fn run(&mut self) {
        loop {
            match self.get_next_command() {
                DebuggerCommand::Run(args) => {
                    if let Some(infer) = self.inferior.as_mut() {
                        infer.kill();
                    }
                    if let Some(inferior) = Inferior::new(&self.target, &args) {
                        self.inferior = Some(inferior);
                        self.cont_command();
                    } else {
                        println!("Error starting subprocess");
                    }
                }
                DebuggerCommand::Quit => {
                    return;
                }
                DebuggerCommand::Cont => self.cont_command(),
                DebuggerCommand::BackTrace => self.back_trace(),
            }
        }
    }

    fn back_trace(&mut self) {
        if self.inferior.is_none() {
            println!("No process is running");
            return;
        }
        let pid = self.inferior.as_ref().unwrap().pid();
        let registers= ptrace::getregs(pid).unwrap();
        let mut instruction_ptr = registers.rip as usize;
        let mut base_ptr = registers.rbp as usize;
        loop {
            let path = self.dwarfData.get_line_from_addr(instruction_ptr).unwrap();
            let func = self.dwarfData.get_function_from_addr(instruction_ptr).unwrap();
            println!("{} ({}:{})", func, path.file, path.number);
            if func == "main" {
                break;
            }
            instruction_ptr = ptrace::read(pid, (base_ptr + 8) as ptrace::AddressType).unwrap() as usize;
            base_ptr = ptrace::read(pid, base_ptr as ptrace::AddressType).unwrap() as usize;
        }
    }

    fn cont_command(&mut self) {
        if self.inferior.is_none() {
            println!("No process is running");
            return;
        }
        ptrace::cont(self.inferior.as_mut().unwrap().pid(), None).ok();
        let status = self.inferior.as_mut().unwrap().wait(None).ok();
        if status.is_none() {
            println!("Child error, no status");
            return;
        }

        let ret = match status.unwrap() {
            crate::inferior::Status::Stopped(sig, _) => format!("Stopped (status {})", sig.as_str()),
            crate::inferior::Status::Exited(code) => {
                self.inferior = None;
                format!("Exited (Status {})", code)
            },
            crate::inferior::Status::Signaled(n) => format!("Signal (Status {})", n.as_str()),
        };
        println!("Child {}", ret);
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
