use bincode::serialize_into;
use csv::{Reader, WriterBuilder};
use csv_read::read::{get_precip_stations, get_temp_stations, get_wind_stations};
use data::{CsvRecord, TemperaturePoint, DataPoint};
use heatmap::HeatMap;
use std::error::Error;
use std::fs::{read_dir, remove_file, File};
use std::io::BufWriter;
use std::path::Path;
use math::{Range, RangeBox, Point};
use glium::backend::glutin::Display;
use glium::draw_parameters::DrawParameters;
use glium::glutin::EventsLoop;
use glium::index::{NoIndices, PrimitiveType::TrianglesList};
use glium::texture::{CompressedSrgbTexture2d, RawImage2d};
use glium::{draw_parameters::Blend, Program, Surface};
use render::{gradient_box, map_box};
use window::Window;
use input;
use image;

pub fn _read_avg_temp(_file_count: usize) {
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
        if i % 1000 == 0 {
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

pub fn read_avg_wind() -> Result<(), Box<Error>> {
    let writer = BufWriter::new(File::create("WindData.bin")?);
    let mut wind_vec = Vec::new();

    for (i, dir) in read_dir("F:/Uni/Grand Challenges/Data/gsom-latest")?.enumerate() {
        if i % 1000 == 1 {
            println!("{}", i);
        }
        let dir = dir.unwrap();
        if let Ok(test) = get_wind_stations(dir.path()) {
            for value in test {
                let point = match TemperaturePoint::from_wind(value) {
                    Some(value) => value,
                    None => continue,
                };
                wind_vec.push(point);
            }
        } else {
            continue;
        }
    }
    serialize_into(writer, &wind_vec)?;
    Ok(())
}

pub fn read_precip() -> Result<(), Box<Error>> {
    let writer = BufWriter::new(File::create("RainData.bin")?);
    let mut rain_vec = Vec::new();

    for (i, dir) in read_dir("F:/Uni/Grand Challenges/Data/gsom-latest")?.enumerate() {
        if i % 1000 == 1 {
            println!("{}", i);
        }
        let dir = dir.unwrap();
        if let Ok(test) = get_precip_stations(dir.path()) {
            for value in test {
                let point = match TemperaturePoint::from_rain(value) {
                    Some(value) => value,
                    None => continue,
                };
                rain_vec.push(point);
            }
        } else {
            continue;
        }
    }
    serialize_into(writer, &rain_vec)?;
    Ok(())
}

pub fn csv_to_bin(path: impl AsRef<Path>) -> Result<(), Box<Error>> {
    let mut reader = Reader::from_path(path)?;
    let mut file = File::create("Data.b")?;

    let mut values = Vec::new();
    for (i, result) in reader.deserialize().enumerate() {
        if i % 1000000 == 0 {
            println!("{}", i);
        }
        let record: CsvRecord = result?;
        let point = match TemperaturePoint::from(record) {
            Some(point) => point,
            None => continue,
        };
        values.push(point);
    }
    serialize_into(&mut file, &values)?;
    Ok(())
}

pub fn write_elevation(path: impl AsRef<Path>) -> Result<(), Box<Error>> {
    let mut reader = Reader::from_path(path)?;
    let mut file = BufWriter::new(File::create("Elevation.b")?);

    let mut values = Vec::new();
    for (i, result) in reader.deserialize().enumerate() {
        if i % 1000000 == 0 {
            println!("{}", i);
        }
        let record: CsvRecord = result?;
        let mut valid = true;
        let long = record.longitude.unwrap_or_else(|| {
            valid = false;
            0.0
        });
        let lat = record.latitude.unwrap_or_else(|| {
            valid = false;
            0.0
        });
        let elevation = record.elevation.unwrap_or_else(|| {
            valid = false;
            0.0
        });

        if !valid {
            continue;
        } 
        let point = DataPoint {
            position: Point::new(long, lat),
            data: elevation
        };
        values.push(point);
    }
    serialize_into(&mut file, &values)?;
    Ok(())
}

pub fn compare_rain_to_standard_dev(
    rain_data: impl AsRef<Path>,
    temp_data: impl AsRef<Path>,
) -> Result<(), Box<Error>> {
    let variance = HeatMap::temp_heat_map_from_bin(
        (720, 360),
        RangeBox::new(
            Range::new(-180.0, 180.0),
            Range::new(-90.0, 90.0),
        ),
        temp_data
    ).unwrap()
        .variance_grid()
        .fill_values_nearest();

    let rain = HeatMap::temp_heat_map_from_bin(
        (720, 360),
        RangeBox::new(
            Range::new(-180.0, 180.0),
            Range::new(-90.0, 90.0),
        ),
        rain_data,
    ).unwrap()
        .average_temp_grid();
    
    let comparison = rain.compare_to(&variance);
    let file = File::create("WindVVar.csv")?;
    let mut wtr = WriterBuilder::new().has_headers(false).from_writer(file);
    wtr.write_record(&[
        "Wind",
        "Var",
    ]).expect("Failed to write headers");

    for value in comparison {
        wtr.serialize(value)?;
    }
    wtr.flush()?;
    Ok(())
}

pub fn compare_elevation_to_deriv_sd(
    elevation: impl AsRef<Path>,
    temp_data: impl AsRef<Path>,
) -> Result<(), Box<Error>> {
    let horizontal = Range::new(-140.0, -50.0);
    let vertical = Range::new(10.0, 60.0);
    let variance = HeatMap::temp_heat_map_from_bin(
        (720, 360),
        RangeBox::new(
            horizontal,
            vertical,
        ),
        temp_data
    ).unwrap()
        .standard_dev_grid()
        .range_grid_from_closest(8)
        .fill_values_nearest();

    let elevation = HeatMap::elevation_map_from_bin(
        (720, 360),
        RangeBox::new(
            horizontal,
            vertical,
        ),
        elevation,
    ).unwrap()
        .elevation_grid();
    
    let comparison = elevation.compare_to(&variance);
    let file = File::create("ElevationVSDD.csv")?;
    let mut wtr = WriterBuilder::new().has_headers(false).from_writer(file);
    wtr.write_record(&[
        "Elevation",
        "SDD",
    ]).expect("Failed to write headers");

    for value in comparison {
        wtr.serialize(value)?;
    }
    wtr.flush()?;
    Ok(())
}

pub fn load_image(display: &Display, path: impl AsRef<Path>) -> CompressedSrgbTexture2d {
    let image = image::open(path).expect("Cannot open image").to_rgba();
    let dims = image.dimensions();
    let raw = RawImage2d::from_raw_rgba_reversed(&image.into_raw(), dims);
    CompressedSrgbTexture2d::new(display, raw).unwrap()
}

pub fn compare_var_to_rain() {
    let mut events_loop = EventsLoop::new();
    let mut window = Window::new(false, true, true, [1800.0, 900.0], &events_loop);
    let program = Program::from_source(
        &window.display,
        include_str!("shaders/vertex.glsl"),
        include_str!("shaders/compare_frag.glsl"),
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

    // let min_pos = [-140.0, 10.0];
    // let max_pos = [-50.0, 60.0];
    let min_pos = [-180.0, -90.0];
    let max_pos = [180.0, 90.0];
    let grid1 = HeatMap::temp_heat_map_from_bin(
        (1200, 600),
        RangeBox::new(
            Range::new(min_pos[0], max_pos[0]),
            Range::new(min_pos[1], max_pos[1]),
        ),
        "Data.bin",
    ).unwrap()
        .standard_dev_grid()
        .fill_values_nearest();

    let grid2 = HeatMap::temp_heat_map_from_bin(
        (1200, 600),
        RangeBox::new(
            Range::new(min_pos[0], max_pos[0]),
            Range::new(min_pos[1], max_pos[1]),
        ),
        "RainData.bin",
    ).unwrap()
        .average_temp_grid()
        .fill_values_nearest();

    let (texture1, range1) = grid1.into_texture(&window.display, Some(Range::new(2.0, 23.0)));
    let (texture2, range2) = grid2.into_texture(&window.display, Some(Range::new(0.0, 350.0)));
    
    println!("range1 = {:?}", range1 );
    println!("range2 = {:?}", range2 );
    
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
            map1: &texture1,
            map2: &texture2,
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

