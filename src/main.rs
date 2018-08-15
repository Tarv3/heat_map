#![allow(dead_code)]

#[macro_use]
extern crate glium;
extern crate csv;
extern crate image;
extern crate serde;
#[macro_use]
extern crate serde_derive;

pub mod csv_read;
pub mod data;
pub mod grid;
pub mod input;
pub mod math;
pub mod render;
pub mod temp_grid;
pub mod window;

use glium::backend::glutin::Display;
use glium::draw_parameters::DrawParameters;
use glium::glutin::EventsLoop;
use glium::index::{
    NoIndices, PrimitiveType::{TrianglesList},
};
use glium::texture::{CompressedSrgbTexture2d, RawImage2d};
use glium::{draw_parameters::Blend, Program, Surface};
use math::{Range, RangeBox};
use render::screen_box;
use std::path::Path;
use temp_grid::{temp_heat_map_from_data};
use window::Window;

use csv::{WriterBuilder};
use csv_read::read::{get_temp_stations};
use std::fs::{read_dir, remove_file, File};

fn main() {
    gl_test();
}

fn gen_random_image(display: &Display, path: impl AsRef<Path>) -> CompressedSrgbTexture2d {
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
    let texture;
    let buffer = screen_box(&window.display);
    {
        let grid = temp_heat_map_from_data((500, 250), RangeBox::new(Range::new(-165.0, -50.0), Range::new(15.0, 60.0)), "Data.csv")
            .unwrap()
            .average_temp_grid();
        texture = grid.into_texture(&window.display);
    }

    let mut contrast = 50.0;
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
            colour1: [0.0, 0.0, 0.0, 1.0f32],
            colour2: [1.0, 1.0, 1.0, 1.0f32],
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
        target.finish().unwrap();
    }
}

fn _reader_test(_file_count: usize) {
    remove_file("Data.csv").expect("Failed to remove data file");

    let buffer = File::create("Data.csv").expect("Couldnt create file");
    let mut wtr = WriterBuilder::new().has_headers(false).from_writer(buffer);
    wtr.write_record(&[
        "YEAR",
        "MONTH",
        "DAY",
        "LONGITUDE",
        "LATITUDE",
        "ELEVATION",
        "TAVG",
    ]).expect("Failed to write headers");
    for (i, dir) in read_dir("F:/Uni/Grand Challenges/Data/gsom-latest")
        .unwrap()
        .enumerate()
    {
        if i % 1000 == 1 {
            println!("{}", i);
        }
        let dir = dir.unwrap();
        if let Ok(test) = get_temp_stations(dir.path()) {
            for value in test {
                wtr.serialize(value).expect("Failed");
            }
        } else {
            continue;
        }
    }
    wtr.flush().expect("Flush failed");
}
