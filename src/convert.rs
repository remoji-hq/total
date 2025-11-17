use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::PathBuf,
};

use clap::{Args, ValueEnum};

const MIN_LEN: usize = 4;

#[derive(Args, Debug)]
pub struct ConvertOptions {
    /// Source data format
    #[clap(short, long, value_enum, value_name = "FORMAT")]
    from: Option<ConvertFrom>,
    /// Destination data format
    #[clap(short, long, value_enum, value_name = "FORMAT")]
    to: ConvertTo,
    /// Output file path
    #[clap(short, long, value_name = "FILE")]
    output: Option<PathBuf>,
    /// Input file path
    #[clap(value_name = "FILE")]
    input: PathBuf,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum ConvertFrom {
    Sdr33Coord,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum ConvertTo {
    Dxf,
}

pub fn run(
    ConvertOptions {
        from: _from,
        to: _to,
        output: _output,
        input,
    }: ConvertOptions,
) -> io::Result<()> {
    let file = File::open(input)?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let Ok(line) = line else {
            continue;
        };
        let line = line.trim();
        if line.len() < MIN_LEN {
            continue;
        }
        if line.starts_with("00NMSDR33") {
            // SDR33
            println!("SDR33 format is not supported yet");
        } else if line.starts_with("00NMSDR20") {
            // SDR2x
            println!("SDR2x format is not supported yet");
            return Ok(());
        } else if line.starts_with("CO,Nikon RAW") {
            // Nikon RAW
            println!("Nikon RAW format is not supported yet");
            return Ok(());
        }
    }

    Ok(())
}
