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
