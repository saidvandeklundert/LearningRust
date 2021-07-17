// Silence some warnings so they don't distract from the exercise.
#![allow(dead_code, unused_variables)]
use core_types;
use core_types::{ding, on_off, print_array, print_distance};

fn main() {
    // Tuple:
    let coords: (f32, f32) = (6.3, 15.0);
    // Pass tuple to func:
    core_types::print_difference(coords.0, coords.1);

    // Array of floats passed into a function:
    let coords_arr: [f32; 2] = [coords.0, coords.1];
    print_array(coords_arr);

    let series: [i32; 7] = [1, 1, 2, 3, 5, 8, 13];
    let last_series = series.len() - 1;
    ding(series[last_series]);

    // pass true from the mess to the on_off function:
    let mess = ([3, 2], 3.14, [(false, -3), (true, -100)], 5, "candy");
    on_off(mess.2[1].0);

    print_distance(coords_arr[0], coords_arr[1]);
}
