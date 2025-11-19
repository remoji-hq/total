use std::io;

use clap::{Parser, Subcommand};

use self::convert::ConvertOptions;

mod convert;
mod format;

/// Total station helper tool
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Command to execute. Options: `convert`
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Convert total station data from one format to another
    Convert(ConvertOptions),
}

fn main() -> io::Result<()> {
    let Args { command } = Args::parse();

    match command {
        Command::Convert(options) => convert::run(options),
    }
}
