use std::{collections::HashMap, io, path::Path};

use dxf::{
    Color, Drawing, DxfError, Point,
    entities::{Circle, Entity, EntityCommon, EntityType, Line, Text},
    enums::{DrawingUnits, UnitFormat, Units},
    tables::Layer,
};

use crate::format::Object;

const POINTS_LAYER: &str = "POINTS";
const TEXT_LAYER: &str = "TEXT";

const COLOR_INDEX_RED: u8 = 1;
const COLOR_INDEX_YELLOW: u8 = 2;
const COLOR_INDEX_GREEN: u8 = 3;

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
        drawing.header.unit_format = UnitFormat::Architectural;
        drawing.header.drawing_units = DrawingUnits::Metric;
        drawing.header.default_drawing_units = Units::Meters;

        drawing.add_layer(Layer {
            name: POINTS_LAYER.to_string(),
            color: Color::from_index(COLOR_INDEX_RED),
            ..Default::default()
        });

        drawing.add_layer(Layer {
            name: TEXT_LAYER.to_string(),
            color: Color::from_index(COLOR_INDEX_YELLOW),
            ..Default::default()
        });

        let mut last_points = HashMap::new();
        let mut color_index = COLOR_INDEX_GREEN;

        for object in objects {
            match object {
                Object::Point {
                    n,
                    e,
                    z,
                    name,
                    code,
                } => {
                    drawing.add_entity(point_circle(n, e, z));
                    if !name.is_empty() {
                        drawing.add_entity(point_text(n, e, z, name));
                    }
                    let layer = if code.is_empty() {
                        "0".to_string()
                    } else {
                        code.clone()
                    };
                    if let Some((last_n, last_e, last_z)) = last_points.get(&code) {
                        drawing.add_entity(Entity {
                            common: EntityCommon {
                                layer,
                                ..Default::default()
                            },
                            specific: EntityType::Line(Line {
                                p1: Point { x: n, y: e, z },
                                p2: Point {
                                    x: *last_n,
                                    y: *last_e,
                                    z: *last_z,
                                },
                                ..Default::default()
                            }),
                        });
                    } else if !code.is_empty() {
                        drawing.add_layer(Layer {
                            name: layer,
                            color: Color::from_index(color_index),
                            ..Default::default()
                        });
                        color_index += 1;
                    }
                    last_points.insert(code, (n, e, z));
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

fn point_text(x: f64, y: f64, z: f64, value: String) -> Entity {
    Entity {
        common: EntityCommon {
            layer: TEXT_LAYER.to_string(),
            ..Default::default()
        },
        specific: EntityType::Text(Text {
            location: Point { x, y, z },
            text_height: 0.5,
            value,
            ..Default::default()
        }),
    }
}

fn point_circle(x: f64, y: f64, z: f64) -> Entity {
    Entity {
        common: EntityCommon {
            layer: POINTS_LAYER.to_string(),
            ..Default::default()
        },
        specific: EntityType::Circle(Circle {
            center: Point { x, y, z },
            radius: 0.1,
            ..Default::default()
        }),
    }
}
