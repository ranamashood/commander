mod command;
mod command_store;
mod db;
mod ui;

use crate::ui::app::App;

use anyhow::Result;
use command::Command;

fn main() -> Result<()> {
    let mut terminal = ratatui::init();
    let app_result = App::default().run(&mut terminal);
    ratatui::restore();
    app_result

    // // let command = Command::new("ls", "List Directories");
    // // println!("{}", command.command());
    // // println!("{}", command.description());
    //
    // // command_store::add(&command)?;
    // let commands = command_store::get_all()?;
    //
    // for command in commands {
    //     println!("{:?}", command);
    // }
    //
    // Ok(())
}
