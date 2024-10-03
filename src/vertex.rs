use beryllium::*;
use ogl33::*;

pub struct vertex<T> {
    pub location: [T; 3],
    pub tex_coords: [f32; 2],
}
