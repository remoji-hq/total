use clap::ValueEnum;

pub use self::dxf::{DxfReader, DxfWriter};
pub use self::nikon::{NikonCoordReader, NikonCoordWriter};
pub use self::sdr::SdrReader;

mod dxf;
mod nikon;
mod sdr;

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum InFormat {
    /// AutoCAD DXF format
    Dxf,
    /// Nikon Coordinates format
    NikonCoord,
    /// Sokkia SDR2x/SDR33 format
    Sdr,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum OutFormat {
    /// AutoCAD DXF format
    Dxf,
    /// Nikon Coordinates format
    NikonCoord,
}

#[derive(Debug, Clone, Copy, ValueEnum, Default)]
pub enum CoordOrder {
    /// East-North-Z order
    #[default]
    ENZ,
    /// North-East-Z order
    NEZ,
}

impl ToString for OutFormat {
    fn to_string(&self) -> String {
        match self {
            Self::Dxf => "dxf".to_string(),
            Self::NikonCoord => "txt".to_string(),
        }
    }
}
