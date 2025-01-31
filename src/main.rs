mod ast;
mod parse;
use ast::ShellCommand;

fn main() -> std::io::Result<()> {
    //println!("Hello, world!");
    let mut buffer = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut buffer)?;
    println!("{}", parse::rushell::exec(&buffer).unwrap().eval(std::process::Stdio::inherit()).unwrap().wait().unwrap());
    Ok(())
}
