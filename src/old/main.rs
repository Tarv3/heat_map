extern crate heat_map;

use heat_map::heatmap::HeatMap;
use heat_map::math::{Range, RangeBox};

fn main() {
    let heat_map = HeatMap::temp_heat_map_from_bin(
        (1200, 600),
        RangeBox::new(Range::new(-180.0, 180.0), Range::new(-90.0, 90.0)),
        "Data.bin"
    ).unwrap();

    let grid = heat_map.into_option_grid().fill_values_nearest();
    grid.save_to_bin("tempgrid.bin").unwrap();
}
