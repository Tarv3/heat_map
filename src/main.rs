#[macro_use]
extern crate glium;
extern crate image;
extern crate csv;

pub mod data;
pub mod grid;
pub mod math;
pub mod render;
pub mod window;
pub mod csv_read;

use render::screen_box;
use data::DataPoint;
use glium::glutin::EventsLoop;
use glium::backend::glutin::Display;
use glium::{Program, Surface};
use glium::uniforms::EmptyUniforms;
use glium::draw_parameters::DrawParameters;
use glium::index::{NoIndices, PrimitiveType::{TrianglesList, LineStrip}};
use glium::texture::{RawImage2d, CompressedSrgbTexture2d, CompressedMipmapsOption};
use grid::{Grid, HeatMap};
use math::{Point, Range, RangeBox};
use window::Window;
use std::path::Path;

fn main() {
    let mut events_loop = EventsLoop::new();
    let mut window = Window::new(false, true, true, [1500.0, 750.0], &events_loop);
    let program = Program::from_source(
        &window.display,
        include_str!("shaders/vertex.glsl"),
        include_str!("shaders/fragment.glsl"),
        None,
    ).unwrap();
    let buffer = screen_box(&window.display);
    let texture = gen_random_image(&window.display, "Birds.jpg"); 
    let uniforms = uniform! {
        map: &texture,
        colour1: [0.0, 0.0, 0.0, 1.0f32],
        colour2: [1.0, 1.0, 1.0, 1.0f32],
    };
    while window.open {
        events_loop.poll_events(|event| {
            window.closer(&event);
        });
        let mut target = window.display.draw();
        target.clear_color(0.0, 0.0, 0.0, 0.0);
        target.draw(&buffer, NoIndices(TrianglesList), &program, &uniforms, &DrawParameters::default()).unwrap();
        target.finish().unwrap();
    }
}
 
fn gen_random_image(display: &Display, path: impl AsRef<Path>) -> CompressedSrgbTexture2d {
    let image = image::open(path).expect("Cannot open image").to_rgba();
    let dims = image.dimensions();
    let raw = RawImage2d::from_raw_rgba_reversed(&image.into_raw(), dims);
    CompressedSrgbTexture2d::new(display, raw).unwrap()
}