use glium::backend::glutin::Display;
use glium::{DrawParameters, Program, VertexBuffer};

#[derive(Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 2],
    pub tex_coord: [f32; 2]
}
implement_vertex!(Vertex, position, tex_coord);

pub fn map_box(display: &Display) -> VertexBuffer<Vertex> {
    VertexBuffer::new(display, &vec![
        Vertex { position: [ -1.0, -1.0 ], tex_coord: [ 0.0, 0.0 ]},
        Vertex { position: [ 0.9, 1.0 ], tex_coord: [ 1.0, 1.0 ]},
        Vertex { position: [ -1.0, 1.0 ], tex_coord: [ 0.0, 1.0 ]},
        Vertex { position: [ -1.0, -1.0 ], tex_coord: [ 0.0, 0.0 ]},
        Vertex { position: [ 0.9, -1.0 ], tex_coord: [ 1.0, 0.0 ]},
        Vertex { position: [ 0.9, 1.0 ], tex_coord: [ 1.0, 1.0 ]},
    ]).unwrap()
}

pub fn gradient_box(display: &Display) -> VertexBuffer<Vertex> {
    VertexBuffer::new(display, &vec![
        Vertex { position: [ 0.9, -1.0 ], tex_coord: [ 0.0, 0.0 ]},
        Vertex { position: [ 1.0, 1.0 ], tex_coord: [ 1.0, 1.0 ]},
        Vertex { position: [ 0.9, 1.0 ], tex_coord: [ 0.0, 1.0 ]},
        Vertex { position: [ 0.9, -1.0 ], tex_coord: [ 0.0, 0.0 ]},
        Vertex { position: [ 1.0, -1.0 ], tex_coord: [ 1.0, 0.0 ]},
        Vertex { position: [ 1.0, 1.0 ], tex_coord: [ 1.0, 1.0 ]},
    ]).unwrap()
}

pub struct RenderSetup<'a> {
    pub program: Program,
    pub draw_params: DrawParameters<'a>,
}

impl<'a> RenderSetup<'a> {
    pub fn new(program: Program, draw_params: DrawParameters<'a>) -> Self {
        Self {
            program,
            draw_params,
        }
    }
    pub fn set_draw_params(&mut self, draw_params: DrawParameters<'a>) {
        self.draw_params = draw_params;
    }
}
