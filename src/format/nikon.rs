use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead, BufReader, Lines, Write},
    path::Path,
};

use crate::geometry::Object;

pub struct NikonCoordReader {
    lines: Lines<BufReader<File>>,
}

pub struct NikonCoordWriter {
    objects: Vec<Object>,
    layers: HashSet<String>,
}

impl NikonCoordReader {
    pub fn new(path: impl AsRef<Path>) -> io::Result<Self> {
        let file = File::open(path.as_ref())?;
        let reader = BufReader::new(file);
        let lines = reader.lines();

        Ok(Self { lines })
    }

    pub fn parse(&mut self) -> Vec<Object> {
        let mut result = Vec::new();

        while let Some(Ok(line)) = self.lines.next() {
            let mut parts = line.trim().split(',');
            let name = parts.next().unwrap_or_default().to_string();
            let n = parts
                .next()
                .and_then(|n| n.parse().ok())
                .unwrap_or_default();
            let e = parts
                .next()
                .and_then(|n| n.parse().ok())
                .unwrap_or_default();
            let z = parts
                .next()
                .and_then(|n| n.parse().ok())
                .unwrap_or_default();
            let code = parts.next().unwrap_or_default().to_string();

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

    pub fn render(&self, path: impl AsRef<Path>) -> io::Result<()> {
        let mut f = File::create(path)?;

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
                        writeln!(&mut f, "{name},{n},{e},{z},{code}")?;
                    }
                }
            }
        }

        Ok(())
    }
}
