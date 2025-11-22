use std::{
    collections::{HashMap, HashSet},
    io,
    path::Path,
};

use dxf::{
    Color, Drawing, DxfError, Point,
    entities::{Circle, Entity, EntityCommon, EntityType, Line, Text},
    enums::{AcadVersion, DrawingUnits, UnitFormat, Units},
    tables::{Layer, ViewPort},
};

use crate::geometry::Object;

const POINTS_LAYER: &str = "POINTS";
const TEXT_LAYER: &str = "TEXT";

const COLOR_INDEX_RED: u8 = 1;
const COLOR_INDEX_YELLOW: u8 = 2;
const COLOR_INDEX_GREEN: u8 = 3;

const TEXT_POINT_PRECISION: f64 = 1000.0; // 1 mm

pub struct DxfReader {
    drawing: Drawing,
}

pub struct DxfWriter {
    drawing: Drawing,
}

impl DxfReader {
    pub fn new(path: impl AsRef<Path>) -> io::Result<Self> {
        let drawing = Drawing::load_file(path).map_err(|e| match e {
            DxfError::IoError(err) => err,
            err => io::Error::other(err),
        })?;
        Ok(Self { drawing })
    }

    pub fn parse(&mut self) -> Vec<Object> {
        let mut result = Vec::new();

        let mut point_names = HashMap::new();
        for entity in self.drawing.entities() {
            match &entity.specific {
                EntityType::Text(text) => {
                    let x = (text.location.x * TEXT_POINT_PRECISION).round() as i32;
                    let y = (text.location.y * TEXT_POINT_PRECISION).round() as i32;
                    point_names.insert((x, y), text.value.clone());
                }
                _ => (),
            }
        }

        let mut number = 1;
        let mut point_name = |x: f64, y: f64| {
            let x = (x * TEXT_POINT_PRECISION).round() as i32;
            let y = (y * TEXT_POINT_PRECISION).round() as i32;
            point_names
                .get(&(x, y))
                .map(Clone::clone)
                .unwrap_or_else(|| {
                    let name = format!("A{number}"); // Auto name
                    number += 1;
                    name
                })
        };

        for entity in self.drawing.entities() {
            let layer = entity.common.layer.trim().to_uppercase();
            match &entity.specific {
                EntityType::Circle(circle) => {
                    result.push(Object::Point {
                        e: circle.center.x,
                        n: circle.center.y,
                        z: circle.center.z,
                        name: point_name(circle.center.x, circle.center.y),
                        code: layer.clone(),
                    });
                }
                EntityType::Line(line) => {
                    result.push(Object::Point {
                        e: line.p1.x,
                        n: line.p1.y,
                        z: line.p1.z,
                        name: point_name(line.p1.x, line.p1.y),
                        code: layer.clone(),
                    });
                    result.push(Object::Point {
                        e: line.p2.x,
                        n: line.p2.y,
                        z: line.p2.z,
                        name: point_name(line.p2.x, line.p2.y),
                        code: layer.clone(),
                    });
                }
                _ => (),
            }
        }

        result
    }
}

impl DxfWriter {
    pub fn new(objects: Vec<Object>, layers: HashSet<String>) -> Self {
        let mut drawing = Drawing::new();
        drawing.header.version = AcadVersion::R2004;
        drawing.header.unit_format = UnitFormat::Architectural;
        drawing.header.drawing_units = DrawingUnits::Metric;
        drawing.header.default_drawing_units = Units::Meters;

        if layers.is_empty() || layers.contains(POINTS_LAYER) {
            drawing.add_layer(Layer {
                name: POINTS_LAYER.to_string(),
                color: Color::from_index(COLOR_INDEX_RED),
                ..Default::default()
            });
        }

        if layers.is_empty() || layers.contains(TEXT_LAYER) {
            drawing.add_layer(Layer {
                name: TEXT_LAYER.to_string(),
                color: Color::from_index(COLOR_INDEX_YELLOW),
                ..Default::default()
            });
        }

        let mut last_points = HashMap::new();
        let mut color_index = COLOR_INDEX_GREEN;

        let mut min_x = None;
        let mut min_y = None;
        let mut max_x = None;
        let mut max_y = None;
        for object in objects {
            match object {
                Object::Point {
                    e,
                    n,
                    z,
                    name,
                    code,
                } => {
                    let x = e;
                    let y = n;

                    min_x = min_x.or(Some(x)).map(|min_x| min_x.min(x));
                    min_y = min_y.or(Some(y)).map(|min_y| min_y.min(y));
                    max_x = max_x.or(Some(x)).map(|max_x| max_x.max(x));
                    max_y = max_y.or(Some(y)).map(|max_y| max_y.max(y));

                    drawing.add_entity(point_circle(x, y, z));
                    if !name.is_empty() {
                        drawing.add_entity(point_text(x, y, z, name));
                    }
                    let layer = if code.is_empty() {
                        "0".to_string()
                    } else {
                        code.clone()
                    };
                    if !layers.is_empty() && !layers.contains(&layer) {
                        continue;
                    }
                    if let Some((last_x, last_y, last_z)) = last_points.get(&code) {
                        drawing.add_entity(Entity {
                            common: EntityCommon {
                                layer,
                                ..Default::default()
                            },
                            specific: EntityType::Line(Line {
                                p1: Point { x, y, z },
                                p2: Point {
                                    x: *last_x,
                                    y: *last_y,
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
                    last_points.insert(code, (x, y, z));
                }
            }
        }

        let max_x = max_x.unwrap_or_default();
        let max_y = max_y.unwrap_or_default();
        let dx = max_x - min_x.unwrap_or_default();
        let dy = max_y - min_y.unwrap_or_default();

        drawing.add_view_port(ViewPort {
            name: "*ACTIVE".to_string(),
            view_center: Point {
                x: max_x / 2.0,
                y: max_y / 2.0,
                z: 0.0,
            },
            view_height: dy,
            view_port_aspect_ratio: dx / dy,
            upper_right: Point {
                x: 1.0,
                y: 1.0,
                z: 0.0,
            },
            ..Default::default()
        });

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
