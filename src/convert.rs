use std::{collections::HashSet, io, path::PathBuf};

use clap::Args;

use crate::format::{
    DxfReader, DxfWriter, InFormat, NikonCoordReader, NikonCoordWriter, OutFormat, SdrReader,
};

#[derive(Args, Debug)]
pub struct ConvertOptions {
    /// Source data format
    #[clap(short, long, value_enum, value_name = "FORMAT")]
    from: InFormat,
    /// Destination data format
    #[clap(short, long, value_enum, value_name = "FORMAT")]
    to: OutFormat,
    /// Filter by layers (codes)
    #[clap(short, long)]
    layers: Vec<String>,
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
        layers,
    }: ConvertOptions,
) -> io::Result<()> {
    let objects = match from {
        InFormat::Dxf => DxfReader::new(&input)?.parse(),
        InFormat::NikonCoord => NikonCoordReader::new(&input)?.parse(),
        InFormat::Sdr => SdrReader::new(&input)?.parse(),
    };

    let output = output.unwrap_or_else(|| input.with_extension(to.to_string()));
    let layers: HashSet<_> = layers.iter().map(|layer| layer.to_uppercase()).collect();
    match to {
        OutFormat::Dxf => DxfWriter::new(objects, layers).render(&output)?,
        OutFormat::NikonCoord => NikonCoordWriter::new(objects, layers).render(&output)?,
    };

    Ok(())
}
