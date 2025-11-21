use clap::ValueEnum;

pub use self::dxf::{DxfReader, DxfWriter};
pub use self::sdr::SdrReader;

mod dxf;
mod sdr;

#[derive(Debug, Clone, Copy, PartialEq, ValueEnum)]
pub enum Format {
    /// AutoCAD DXF format
    Dxf,
    /// Sokkia SDR2x format
    Sdr2x,
    /// Sokkia SDR33 format
    Sdr33,
}

#[derive(Debug)]
pub enum Object {
    Point {
        e: f64,
        n: f64,
        z: f64,
        name: String,
        code: String,
    },
}

impl ToString for Format {
    fn to_string(&self) -> String {
        match self {
            Format::Dxf => "dxf".to_string(),
            Format::Sdr2x | Format::Sdr33 => "sdr".to_string(),
        }
    }
}
