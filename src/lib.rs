pub mod bucket;
pub mod max_rects;
pub mod packing_box;
pub mod visualizer;

use bucket::Bucket;
use packing_box::PackingBox;

pub fn calculate_packed_percentage(placed_boxes: &[PackingBox], bins: &[Bucket]) -> f32 {
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
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_packed_percentage_no_bins() {
        let placed_boxes = vec![PackingBox::new(5, 5), PackingBox::new(10, 10)];
        let bins = vec![];
        let percentage = calculate_packed_percentage(&placed_boxes, &bins);
        assert_eq!(percentage, 0.0);
    }

    #[test]
    fn test_packed_percentage_no_boxes() {
        let placed_boxes = vec![];
        let bins = vec![Bucket::new(5, 5, 0, 0, 1), Bucket::new(10, 10, 0, 0, 2)];
        let percentage = calculate_packed_percentage(&placed_boxes, &bins);
        assert_eq!(percentage, 0.0);
    }

    #[test]
    fn test_packed_percentage_filled() {
        let placed_boxes = vec![
            PackingBox::new(5, 5),
            PackingBox::new(10, 10),
            PackingBox::new(5, 5),
            PackingBox::new(5, 5),
            PackingBox::new(5, 5),
            PackingBox::new(5, 5),
        ];
        let bins = vec![Bucket::new(15, 15, 0, 0, 1)];
        let percentage = calculate_packed_percentage(&placed_boxes, &bins);
        assert_eq!(percentage, 100.0);
    }

    #[test]
    fn test_packed_percentage_half_filled() {
        let placed_boxes = vec![PackingBox::new(5, 5)];
        let bins = vec![Bucket::new(10, 10, 0, 0, 1)];
        let percentage = calculate_packed_percentage(&placed_boxes, &bins);
        assert_eq!(percentage, 25.0);
    }
}
