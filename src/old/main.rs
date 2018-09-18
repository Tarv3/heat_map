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
pub mod heatmap;

use data::YearlyData;
use glium::backend::glutin::Display;
use glium::draw_parameters::DrawParameters;
use glium::glutin::EventsLoop;
use glium::index::{NoIndices, PrimitiveType::TrianglesList};
use glium::{draw_parameters::Blend, Program, Surface};
use grid::Grid;
use heatmap::HeatMap;
use helper::*;
use math::{Range, RangeBox};
use render::{gradient_box, map_box};
use std::path::Path;
use window::Window;

fn main() {
    // read_precip().expect("failed to read wind");
    render_heatmap();
    // compare_elevation_to_deriv_sd("Elevation.bin", "Data.bin");
    // compare_rain_to_standard_dev("WindData.bin", "Data.b");
}

fn load_programs(display: &Display) -> (Program, Program) {
    let program = Program::from_source(
        display,
        include_str!("shaders/vertex.glsl"),
        include_str!("shaders/fragment.glsl"),
        None,
    ).unwrap();
    let grad_program = Program::from_source(
        display,
        include_str!("shaders/vertex.glsl"),
        include_str!("shaders/gradient.glsl"),
        None,
    ).unwrap();
    return (program, grad_program);
}

fn elevation_heatmap(
    dimensions: (usize, usize),
    horizontal: Range<f32>,
    vertical: Range<f32>,
) -> Grid<Option<f32>> {
    let grid = HeatMap::elevation_map_from_bin(
        dimensions,
        RangeBox::new(horizontal, vertical),
        "Elevation.bin",
    );
    grid.unwrap().elevation_grid()
}

fn render_heatmap() {
    let mut events_loop = EventsLoop::new();
    let mut window = Window::new(false, true, true, [1800.0, 900.0], &events_loop);
    let (program, grad_program) = load_programs(&window.display);
    let buffer = map_box(&window.display);
    let grad_buffer = gradient_box(&window.display);

    // US
    // let horizontal = Range::new(-140.0, -50.0);
    // let vertical = Range::new(10.0, 60.0);
    // Aus
    // let horizontal = Range::new(110.0, 155.0);
    // let vertical = Range::new(-44.0, -8.0);
    // Europe
    // let horizontal = Range::new(-30.0, 75.0);
    // let vertical = Range::new(15.0, 75.0);

    let horizontal = Range::new(-180.0, 180.0);
    let vertical = Range::new(-90.0, 90.0);

    let grid = HeatMap::temp_heat_map_from_bin((1200, 600), RangeBox::new(horizontal, vertical), "Data.bin");
    let grid = grid.unwrap().standard_dev_grid().fill_values_nearest();
    // let grid = elevation_heatmap((1200, 600), horizontal, vertical).fill_values_nearest();
    
    let (texture, range) = grid.into_texture(&window.display, None);
    println!("range = {:?}", range);

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
            min_pos: [horizontal.from, vertical.from],
            max_pos: [horizontal.to, vertical.to],
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
