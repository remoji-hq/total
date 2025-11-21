use std::{io, path::PathBuf};

use clap::Args;

use crate::format::{DxfReader, DxfWriter, InFormat, OutFormat, SdrReader};

#[derive(Args, Debug)]
pub struct ConvertOptions {
    /// Source data format
    #[clap(short, long, value_enum, value_name = "FORMAT")]
    from: InFormat,
    /// Destination data format
    #[clap(short, long, value_enum, value_name = "FORMAT")]
    to: OutFormat,
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
    let objects = match from {
        InFormat::Dxf => DxfReader::new(&input)?.parse(),
        InFormat::Sdr => SdrReader::new(&input)?.parse(),
    };

    let output = output.unwrap_or_else(|| input.with_extension(to.to_string()));
    match to {
        OutFormat::Dxf => DxfWriter::new(objects).render(&output)?,
    };

    Ok(())
}
