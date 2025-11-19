use std::{io, path::Path};

use dxf::{
    Drawing, DxfError, Point,
    entities::{Entity, EntityType, Line, Text},
};

use crate::format::Object;

pub struct DxfReader;

pub struct DxfWriter {
    drawing: Drawing,
}

impl DxfReader {
    pub fn new(_path: impl AsRef<Path>) -> io::Result<Self> {
        Ok(Self)
    }

    pub fn parse(&mut self) -> Vec<Object> {
        println!("Not implemented yet");
        vec![]
    }
}

impl DxfWriter {
    pub fn new(objects: Vec<Object>) -> Self {
        let mut drawing = Drawing::new();

        let mut last_point = None;
        for object in objects {
            match object {
                Object::Point { n, e, z, name, .. } => {
                    if !name.is_empty() {
                        drawing.add_entity(Entity::new(EntityType::Text(Text {
                            location: Point { x: n, y: e, z },
                            text_height: 1.0,
                            value: name,
                            ..Default::default()
                        })));
                    }
                    if let Some((ln, le, lz)) = last_point {
                        drawing.add_entity(Entity::new(EntityType::Line(Line {
                            p1: Point { x: n, y: e, z },
                            p2: Point {
                                x: ln,
                                y: le,
                                z: lz,
                            },
                            ..Default::default()
                        })));
                    }
                    last_point = Some((n, e, z));
                }
            }
        }

        Self { drawing }
    }

    pub fn render(&self, path: impl AsRef<Path>) -> io::Result<()> {
        self.drawing.save_file(path.as_ref()).map_err(|e| match e {
            DxfError::IoError(err) => err,
            err => io::Error::other(err),
        })
    }
}
