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