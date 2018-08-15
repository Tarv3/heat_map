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
pub mod math;
pub mod render;
pub mod temp_grid;
pub mod window;
pub mod input;

use data::DataPoint;
use glium::backend::glutin::Display;
use glium::draw_parameters::DrawParameters;
use glium::glutin::EventsLoop;
use glium::index::{
    NoIndices, PrimitiveType::{LineStrip, Points, TrianglesList},
};
use glium::texture::{CompressedMipmapsOption, CompressedSrgbTexture2d, RawImage2d};
use glium::uniforms::EmptyUniforms;
use glium::{draw_parameters::Blend, Program, Surface};
use grid::{Grid, HeatMap};
use math::{Point, Range, RangeBox};
use render::screen_box;
use std::path::Path;
use temp_grid::{point_buffer, temp_heat_map_from_data, TemperatureGrid};
use window::Window;

use csv::{Writer, WriterBuilder};
use csv_read::read::{get_temp_stations, TempStation};
use std::fs::{read_dir, remove_file, File};
use std::io::prelude::Write;

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
    let buffer = screen_box(&window.display);
    // let points = point_buffer(&window.display, "Data.csv").unwrap();
    // let texture = gen_random_image(&window.display, "Birds.jpg");
    let grid = temp_heat_map_from_data(400, 200, "Data.csv")
        .unwrap()
        .average_temp_grid();
    let texture = grid.into_texture(&window.display);


    let mut contrast = 50.0;
    let uniforms = uniform! {
        map: &texture,
        colour1: [0.0, 0.0, 0.0, 1.0f32],
        colour2: [1.0, 1.0, 1.0, 1.0f32],
        contrast: contrast
    };
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

fn reader_test(file_count: usize) {
    remove_file("Data.csv").expect("Failed to remove data file");

    let mut buffer = File::create("Data.csv").expect("Couldnt create file");
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
