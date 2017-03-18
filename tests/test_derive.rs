extern crate glium;
#[macro_use]
extern crate glium_derive;

#[derive(Copy,Clone,GliumVertex)]
struct MyVertex<T: Copy>
    where [T;2]: glium::vertex::Attribute,
          [T;3]: glium::vertex::Attribute
{
    pos: [T; 2],
    color: [T; 3]
}

fn main() {}
