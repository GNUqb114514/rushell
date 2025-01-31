use crate::ast::ExecCommand;

peg::parser! {
    pub grammar rushell() for str {
        rule string() -> String
            = "\"" s:[^ '"' | '\n']* "\"" {s.into_iter().collect()}
        rule identifier() -> String
            = s:string() {s}
            / s:[^ ' ' | '\t' | '\n']+ {s.into_iter().collect()}
        rule noargs() -> ExecCommand
            = program:identifier() { ExecCommand::new(program, vec![]) }
        rule withargs() -> ExecCommand
            = program:identifier() " " args:identifier()**[' ' | '\t'] { ExecCommand::new(program, args) }
        pub rule exec() -> ExecCommand
            = command:noargs() "\n" { command }
            / command:withargs() "\n" { command }
    }
}
