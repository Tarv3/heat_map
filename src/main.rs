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
pub mod window;

use data::DataPoint;
use glium::backend::glutin::Display;
use glium::draw_parameters::DrawParameters;
use glium::glutin::EventsLoop;
use glium::index::{
    NoIndices, PrimitiveType::{LineStrip, TrianglesList},
};
use glium::texture::{CompressedMipmapsOption, CompressedSrgbTexture2d, RawImage2d};
use glium::uniforms::EmptyUniforms;
use glium::{Program, Surface};
use grid::{Grid, HeatMap};
use math::{Point, Range, RangeBox};
use render::screen_box;
use std::path::Path;
use window::Window;

use csv_read::read::{get_temp_stations, TempStation};
use std::fs::{read_dir, remove_file, File};
use std::io::prelude::Write;
use csv::{Writer, WriterBuilder};

fn main() {
    reader_test(20000);
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
        target
            .draw(
                &buffer,
                NoIndices(TrianglesList),
                &program,
                &uniforms,
                &DrawParameters::default(),
            )
            .unwrap();
        target.finish().unwrap();
    }
}

fn reader_test(file_count: usize) {
    remove_file("Data.csv");

    let mut buffer = File::create("Data.csv").expect("Couldnt create file");
    let mut wtr = WriterBuilder::new().has_headers(false).from_writer(buffer);
    wtr.write_record(&["YEAR", "MONTH", "DAY", "LONGITUDE", "LATITUDE", "ELEVATION", "TAVG"]);
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
        }
        else {
            continue;
        }

    }
    wtr.flush().expect("Flush failed");
}
