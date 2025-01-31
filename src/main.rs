mod ast;
mod error;
mod parse;
use ast::ShellCommand;

fn repl_once() -> error::Result<()> {
    let mut buffer = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut buffer)?;
    println!(
        "{}",
        parse::rushell::exec(&buffer)?
            .eval(std::process::Stdio::inherit())?
            .wait()?
    );
    Ok(())
}

fn main() -> error::Result<()> {
    loop {
        match repl_once() {
            Ok(()) => {}
            Err(err) => println!("{}", err),
        }
    }
}
