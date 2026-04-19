use std::{collections::HashSet, io, path::PathBuf};

use clap::Args;

use crate::format::{
    CoordOrder, DxfReader, DxfWriter, InFormat, NikonCoordReader, NikonCoordWriter, OutFormat,
    SdrReader,
};

#[derive(Args, Debug)]
pub struct ConvertOptions {
    /// Source data format
    #[arg(short, long, value_enum, value_name = "FORMAT")]
    from: InFormat,
    /// Destination data format
    #[arg(short, long, value_enum, value_name = "FORMAT")]
    to: OutFormat,
    /// Filter by layers (codes)
    #[arg(short, long)]
    layers: Vec<String>,
    /// Output file path
    #[arg(short, long, value_name = "FILE")]
    output: Option<PathBuf>,
    /// Set the default coordinate order
    #[arg(long)]
    coord_order: Option<CoordOrder>,
    /// Input file path
    #[arg(value_name = "FILE")]
    input: PathBuf,
}

pub fn run(
    ConvertOptions {
        from,
        to,
        output,
        input,
        layers,
        coord_order,
    }: ConvertOptions,
) -> io::Result<()> {
    let objects = match from {
        InFormat::Dxf => DxfReader::new(&input)?.parse(),
        InFormat::NikonCoord => NikonCoordReader::new(&input, coord_order)?.parse(),
        InFormat::Sdr => SdrReader::new(&input, coord_order)?.parse(),
    };

    let output = output.unwrap_or_else(|| input.with_extension(to.to_string()));
    let layers: HashSet<_> = layers.iter().map(|layer| layer.to_uppercase()).collect();
    match to {
        OutFormat::Dxf => DxfWriter::new(objects, layers).render(&output)?,
        OutFormat::NikonCoord => {
            NikonCoordWriter::new(objects, layers).render(&output, coord_order)?
        }
    };

    Ok(())
}
