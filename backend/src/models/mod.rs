use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Entry {
    pub x: i32,
    pub y: i32
}

#[derive(Deserialize, Serialize)]
pub struct Bounds {
    pub x_min: i32,
    pub x_max: i32,
    pub y_min: i32,
    pub y_max: i32,
}
