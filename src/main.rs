mod command;
mod command_store;
mod db;

use anyhow::Result;
use command::Command;

fn main() -> Result<()> {
    // let command = Command::new("ls", "List Directories");
    // println!("{}", command.command());
    // println!("{}", command.description());

    // command_store::add(&command)?;
    let commands = command_store::get_all()?;

    for command in commands {
        println!("{:?}", command);
    }

    Ok(())
}
