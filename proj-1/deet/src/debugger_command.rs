pub enum DebuggerCommand {
    Quit,
    Run(Vec<String>),
    Cont,
    BackTrace,
    BreakPoint(Option<String>),
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
            "b" | "breakpoint" => {
                let pos = match tokens.get(1) {
                    Some(s) => Some(s.to_string()),
                    None => None,
                };
                Some(DebuggerCommand::BreakPoint(pos))
            }
            _ => None,
        }
    }
}
