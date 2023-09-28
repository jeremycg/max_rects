extern crate rand;
use rand::Rng;

use clap::{App, Arg};

mod bucket;
mod max_rects;
mod packing_box;
mod visualizer;

use bucket::Bucket;
use max_rects::MaxRects;
use packing_box::PackingBox;
use visualizer::generate_visualization;

/// Validates if a string can be parsed to an i32.
///
/// # Arguments
/// - `val`: The string value to be parsed.
///
/// # Returns
/// - A `Result` indicating success or failure.
fn is_i32(val: &str) -> Result<(), String> {
    val.parse::<i32>()
        .map(|_| ())
        .map_err(|err| format!("could not parse '{}' as i32: {}", val, err))
}

/// Parses command line arguments.
///
/// # Returns
/// - A tuple containing two i32 values representing the number of boxes and bins respectively.
fn parse_arguments() -> (i32, i32) {
    let matches = App::new("Box Packing")
        .version("1.0")
        .about("Generates a visualization based on the number of boxes and bins")
        .arg(
            Arg::new("boxes")
                .short('b')
                .long("boxes")
                .value_name("NUMBER")
                .takes_value(true)
                .required(true)
                .validator(is_i32)
                .help("Sets the number of boxes to place"),
        )
        .arg(
            Arg::new("bins")
                .short('n')
                .long("bins")
                .value_name("NUMBER")
                .takes_value(true)
                .required(true)
                .validator(is_i32)
                .help("Sets the number of bins to pack"),
        )
        .get_matches();

    let num_boxes = matches.value_of("boxes").unwrap().parse::<usize>().unwrap() as i32;
    let num_bins = matches.value_of("bins").unwrap().parse::<usize>().unwrap() as i32;

    (num_boxes, num_bins)
}

fn calculate_packed_percentage(placed_boxes: &[PackingBox], bins: &[Bucket]) -> f32 {
    // Summing up the area of all bins
    let total_bin_area: i32 = bins.iter().map(|bin| bin.width * bin.height).sum();

    // Summing up the area of all placed boxes
    let total_placed_box_area: i32 = placed_boxes
        .iter()
        .map(|box_item| box_item.width * box_item.height)
        .sum();

    // Checking for zero to prevent division by zero
    if total_bin_area == 0 {
        return 0.0;
    }

    // Calculating the percentage
    (total_placed_box_area as f32 / total_bin_area as f32) * 100.0
}

fn main() {
    let (num_boxes, num_bins) = parse_arguments();

    let mut rng = rand::thread_rng();
    let mut bins: Vec<Bucket> = Vec::new();
    for idx in 0..num_bins {
        let new_bin = Bucket::new(200, 200, 0, 0, idx);
        bins.push(new_bin);
    }

    let mut boxes: Vec<PackingBox> = Vec::new();

    for _ in 0..num_boxes {
        let width = rng.gen_range(1..100);
        let height = rng.gen_range(1..100);

        let new_box = PackingBox::new(width, height);
        boxes.push(new_box);
    }
    let bins_clone = bins.clone();
    let mut problem = MaxRects::new(boxes, bins);
    let (placed, missed, remaining_bins) = problem.place();
    generate_visualization(&placed, &bins_clone);

    println!("Placed: {:?}", placed);
    println!("Missed: {:?}", missed);
    println!("Remaining Bins: {:?}", remaining_bins);
    let percentage_packed = calculate_packed_percentage(&placed, &bins_clone);
    println!("Percentage Packed: {:.2}%", percentage_packed);
}
