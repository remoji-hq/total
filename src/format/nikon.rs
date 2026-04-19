use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead, BufReader, Lines, Write},
    path::Path,
};

use crate::{format::CoordOrder, geometry::Object};

pub struct NikonCoordReader {
    coord_order: CoordOrder,
    lines: Lines<BufReader<File>>,
}

pub struct NikonCoordWriter {
    objects: Vec<Object>,
    layers: HashSet<String>,
}

impl NikonCoordReader {
    pub fn new(path: impl AsRef<Path>, coord_order: Option<CoordOrder>) -> io::Result<Self> {
        let file = File::open(path.as_ref())?;
        let reader = BufReader::new(file);
        let lines = reader.lines();

        Ok(Self {
            coord_order: coord_order.unwrap_or_default(),
            lines,
        })
    }

    pub fn parse(&mut self) -> Vec<Object> {
        let mut result = Vec::new();

        while let Some(Ok(line)) = self.lines.next() {
            let mut parts = line.trim().split(',');
            let name = parts.next().unwrap_or_default().to_string();
            let a = parts
                .next()
                .and_then(|n| n.parse().ok())
                .unwrap_or_default();
            let b = parts
                .next()
                .and_then(|n| n.parse().ok())
                .unwrap_or_default();
            let c = parts
                .next()
                .and_then(|n| n.parse().ok())
                .unwrap_or_default();
            let code = parts.next().unwrap_or_default().to_string();
            let (e, n, z) = match self.coord_order {
                CoordOrder::ENZ => (a, b, c),
                CoordOrder::NEZ => (b, a, c),
            };
            result.push(Object::Point {
                e,
                n,
                z,
                name,
                code,
            });
        }

        result
    }
}

impl NikonCoordWriter {
    pub fn new(objects: Vec<Object>, layers: HashSet<String>) -> Self {
        Self { objects, layers }
    }

    pub fn render(
        &self,
        path: impl AsRef<Path>,
        coord_order: Option<CoordOrder>,
    ) -> io::Result<()> {
        let mut f = File::create(path)?;
        let coord_order = coord_order.unwrap_or_default();

        for object in &self.objects {
            match object {
                Object::Point {
                    e,
                    n,
                    z,
                    name,
                    code,
                } => {
                    if self.layers.is_empty() || self.layers.contains(code) {
                        match coord_order {
                            CoordOrder::ENZ => writeln!(&mut f, "{name},{e},{n},{z},{code}")?,
                            CoordOrder::NEZ => writeln!(&mut f, "{name},{n},{e},{z},{code}")?,
                        }
                    }
                }
            }
        }

        Ok(())
    }
}
