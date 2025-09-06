#[derive(Debug)]
pub struct Command {
    command: String,
    description: String,
}

impl Command {
    pub fn new(command: String, description: String) -> Self {
        Self {
            command,
            description,
        }
    }

    pub fn command(&self) -> &str {
        &self.command
    }

    pub fn description(&self) -> &str {
        &self.description
    }
}
