use std::{
    fs::File,
    io::{self, BufRead, BufReader, Lines},
    path::Path,
};

use crate::geometry::Object;

pub struct SdrReader {
    version: SdrVersion,
    lines: Lines<BufReader<File>>,
}

enum SdrVersion {
    Sdr33,
    Sdr2x,
}

impl SdrReader {
    pub fn new(path: impl AsRef<Path>) -> io::Result<Self> {
        let file = File::open(path.as_ref())?;
        let reader = BufReader::new(file);
        let mut lines = reader.lines();
        let version = loop {
            match lines.next() {
                Some(Ok(line)) => {
                    if line.starts_with("00NMSDR33") {
                        break SdrVersion::Sdr33;
                    } else if line.starts_with("00NMSDR20") {
                        break SdrVersion::Sdr2x;
                    }
                }
                Some(Err(e)) => return Err(e),
                None => {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidData,
                        "Failed to detect SDR version: no header found",
                    ));
                }
            }
        };

        Ok(Self { version, lines })
    }

    pub fn parse(&mut self) -> Vec<Object> {
        let mut result = Vec::new();

        while let Some(Ok(line)) = self.lines.next() {
            let line = line.trim();
            if line.starts_with("08KI") {
                let (name, n, e, z, code) = match self.version {
                    SdrVersion::Sdr2x => (
                        line.get(4..8).unwrap_or_default(),
                        get_f64(line.get(8..18)),
                        get_f64(line.get(18..28)),
                        get_f64(line.get(28..38)),
                        line.get(38..).unwrap_or_default(),
                    ),
                    SdrVersion::Sdr33 => (
                        line.get(4..20).unwrap_or_default(),
                        get_f64(line.get(20..36)),
                        get_f64(line.get(36..52)),
                        get_f64(line.get(52..68)),
                        line.get(68..).unwrap_or_default(),
                    ),
                };
                result.push(Object::Point {
                    e,
                    n,
                    z,
                    name: name.trim().to_string(),
                    code: code.trim().to_string(),
                });
            }
        }

        result
    }
}

#[inline]
fn get_f64(coord: Option<&str>) -> f64 {
    coord
        .map(str::trim)
        .and_then(|n| n.parse().ok())
        .unwrap_or_default()
}
