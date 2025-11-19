use std::{io, path::PathBuf};

use clap::Args;

use crate::format::{DxfReader, DxfWriter, Format, SdrReader};

#[derive(Args, Debug)]
pub struct ConvertOptions {
    /// Source data format
    #[clap(short, long, value_enum, value_name = "FORMAT")]
    from: Format,
    /// Destination data format
    #[clap(short, long, value_enum, value_name = "FORMAT")]
    to: Format,
    /// Output file path
    #[clap(short, long, value_name = "FILE")]
    output: Option<PathBuf>,
    /// Input file path
    #[clap(value_name = "FILE")]
    input: PathBuf,
}

pub fn run(
    ConvertOptions {
        from,
        to,
        output,
        input,
    }: ConvertOptions,
) -> io::Result<()> {
    if from == to {
        println!("Source and destination formats can't be the same");
        return Ok(());
    }
    let objects = match from {
        Format::Dxf => DxfReader::new(&input)?.parse(),
        Format::Sdr2x | Format::Sdr33 => SdrReader::new(&input)?.parse(),
    };

    let output = output.unwrap_or_else(|| input.with_extension(to.to_string()));
    match to {
        Format::Dxf => DxfWriter::new(objects).render(&output)?,
        Format::Sdr2x => todo!(),
        Format::Sdr33 => todo!(),
    };

    Ok(())
}
