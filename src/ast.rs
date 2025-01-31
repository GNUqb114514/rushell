use std::process::{Child, Command, Stdio};

use crate::error::{Result, RushellError::CommandNotFound};

pub trait ShellCommand {
    fn eval(&self, stdin: Stdio) -> Result<Child>;
}

pub struct ExecCommand {
    program: String,
    args: Vec<String>,
}

impl ExecCommand {
    pub fn new(program: String, args: Vec<String>) -> Self {
        Self { program, args }
    }
}

impl ShellCommand for ExecCommand {
    fn eval(&self, stdin: Stdio) -> Result<Child> {
        Command::new(&self.program)
            .args(&self.args)
            .stdin(stdin)
            .spawn()
            .map_err(|err| match err.kind() {
                std::io::ErrorKind::NotFound => {
                    CommandNotFound(self.program.clone())
                }
                _ => err.into(),
            })
    }
}
