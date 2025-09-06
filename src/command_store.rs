use anyhow::Result;

use crate::Command;
use crate::db;

pub fn add(command: &Command) -> Result<()> {
    let db = db::get_db()?;

    db.execute(
        "INSERT INTO commands(command, description) VALUES(?1, ?2)",
        (command.command(), command.description()),
    )?;

    Ok(())
}

pub fn get_all() -> Result<Vec<Command>> {
    let db = db::get_db()?;
    let mut stmt = db.prepare("SELECT command, description FROM commands")?;

    let mut commands = Vec::new();

    let command_iter = stmt.query_map([], |row| Ok(Command::new(row.get(0)?, row.get(1)?)))?;

    for command in command_iter {
        commands.push(command?);
    }

    Ok(commands)
}
