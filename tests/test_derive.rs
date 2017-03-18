#[macro_use]
extern crate glium;
#[macro_use]
extern crate glium_derive;

use glium::vertex::Vertex;

#[derive(GliumVertex)]
struct MyVertex {
    pos: [f32; 2],
    color: [f32; 3]
}

fn main() {}
