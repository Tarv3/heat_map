#![allow(dead_code)]

#[macro_use]
extern crate glium;
extern crate csv;
extern crate image;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate bincode;
extern crate rayon;

pub mod csv_read;
pub mod data;
pub mod grid;
pub mod helper;
pub mod input;
pub mod math;
pub mod render;
pub mod window;

use glium::backend::glutin::Display;
use glium::draw_parameters::DrawParameters;
use glium::glutin::EventsLoop;
use glium::index::{NoIndices, PrimitiveType::TrianglesList};
use glium::texture::{CompressedSrgbTexture2d, RawImage2d};
use glium::{draw_parameters::Blend, Program, Surface};
use grid::HeatMap;
use math::{Range, RangeBox};
use render::{gradient_box, map_box};
use std::path::Path;
use window::Window;

use csv::WriterBuilder;
use csv_read::read::get_temp_stations;
use helper::csv_to_bin;
use std::fs::{read_dir, remove_file, File};

fn main() {
    gl_test();
}

fn load_image(display: &Display, path: impl AsRef<Path>) -> CompressedSrgbTexture2d {
    let image = image::open(path).expect("Cannot open image").to_rgba();
    let dims = image.dimensions();
    let raw = RawImage2d::from_raw_rgba_reversed(&image.into_raw(), dims);
    CompressedSrgbTexture2d::new(display, raw).unwrap()
}

fn gl_test() {
    let mut events_loop = EventsLoop::new();
    let mut window = Window::new(false, true, true, [1500.0, 750.0], &events_loop);
    let program = Program::from_source(
        &window.display,
        include_str!("shaders/vertex.glsl"),
        include_str!("shaders/fragment.glsl"),
        None,
    ).unwrap();
    let grad_program = Program::from_source(
        &window.display,
        include_str!("shaders/vertex.glsl"),
        include_str!("shaders/gradient.glsl"),
        None,
    ).unwrap();
    let buffer = map_box(&window.display);
    let grad_buffer = gradient_box(&window.display);

    let min_pos = [-140.0, 10.0];
    let max_pos = [-50.0, 60.0];
    // let min_pos = [-180.0, -90.0];
    // let max_pos = [180.0, 90.0];
    let grid = HeatMap::temp_heat_map_from_bin(
        (720, 360),
        RangeBox::new(
            Range::new(min_pos[0], max_pos[0]),
            Range::new(min_pos[1], max_pos[1]),
        ),
        "Data.b",
    ).unwrap()
        .standard_dev_grid()
        .fill_values_nearest();

    let (texture, range) = grid.into_texture(&window.display, Some(Range::new(2.0, 23.0)));
    
    println!("range = {:?}", range );
    
    let map_texture = load_image(&window.display, "Pure B and W Map.png");
    let mut contrast = 0.0;
    let draw_parameters = DrawParameters {
        blend: Blend::alpha_blending(),
        ..Default::default()
    };

    while window.open {
        events_loop.poll_events(|event| {
            window.closer(&event);
            input::take_input(&event, &mut contrast)
        });
        let uniforms = uniform! {
            map: &texture,
            bwmap: &map_texture,
            min_pos: min_pos,
            max_pos: max_pos,
            contrast: contrast
        };
        let mut target = window.display.draw();
        target.clear_color(0.0, 0.0, 0.0, 0.0);
        target
            .draw(
                &buffer,
                NoIndices(TrianglesList),
                &program,
                &uniforms,
                &draw_parameters,
            )
            .unwrap();
        let uniforms = uniform! {
            contrast: contrast
        };

        target
            .draw(
                &grad_buffer,
                NoIndices(TrianglesList),
                &grad_program,
                &uniforms,
                &draw_parameters,
            )
            .unwrap();
        target.finish().unwrap();
    }
}
