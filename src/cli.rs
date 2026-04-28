use crate::command::Commands;

pub struct Cli {
    pub command: Commands,
}

impl Cli {
    pub fn parse() -> Option<Self> {
        let mut args = std::env::args().skip(1);
        while let Some(arg) = args.next() {
            if let Some(command) = Commands::from(&arg, &mut args) {
                return Some(Cli { command });
            }
        }
        None
    }
}
