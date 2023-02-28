pub enum DebuggerCommand {
    Quit,
    Run(Vec<String>),
    Cont,
    BackTrace,
}

impl DebuggerCommand {
    pub fn from_tokens(tokens: &Vec<&str>) -> Option<DebuggerCommand> {
        match tokens[0] {
            "q" | "quit" => Some(DebuggerCommand::Quit),
            "r" | "run" => {
                let args = tokens[1..].to_vec();
                Some(DebuggerCommand::Run(
                    args.iter().map(|s| s.to_string()).collect(),
                ))
            }
            "c" | "cont" =>  Some(DebuggerCommand::Cont),
            "bt" | "back" | "backtrace" =>  Some(DebuggerCommand::BackTrace),
            _ => None,
        }
    }
}
