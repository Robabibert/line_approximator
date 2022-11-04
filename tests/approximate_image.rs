use htree::HTree;
use image::{self, ImageBuffer, Luma, DynamicImage};
use imageproc::{drawing::{draw_line_segment_mut, draw_filled_rect_mut, draw_antialiased_line_segment}, rect::Rect};
use line_approximator::{
    approximate,
    line_utils::{length, partition_line, thicken_line},
};

#[test]
fn approximate_image() {
    let target_image:ImageBuffer<Luma<u8>, Vec<u8>> = image::open("resources/rust_logo.png").unwrap().to_luma8();
    let mut approximated_image: ImageBuffer<Luma<u8>, Vec<u8>> = ImageBuffer::new(target_image.width(), target_image.height());
    // white background
    approximated_image.fill(255u8);

    let (width, height) = (target_image.width() as f32, target_image.height() as f32);
    let htree: HTree<f32> = HTree::new(10);

    //width, height of original_htree : [1,1/sqrt(2)]
    //scale htree to image size
    let lines: Vec<((f32, f32), (f32, f32))> = htree
        .into_iter()
        .map(|(start, stop)| {
            (
                (start.0 * width, start.1 * height * 2f32.sqrt()),
                (stop.0 * width, stop.1 * height * 2f32.sqrt()),
            )
        })
        .collect();
    let total_line_length = lines
        .iter()
        .map(|(start, stop)| length(start, stop))
        .sum::<f32>();

    //approximate maximum thickness
    let max_thickness = width * height / total_line_length;
    let black = Luma([0u8]);
    for line in lines
        .iter()
        .map(|line| partition_line(&line.0, &line.1, 1f32))
        .flatten()
        .map(|line| {
            let thickness=approximate(&target_image, &line.0, &line.1, max_thickness);
            thicken_line(&line.0, &line.1, thickness)
        }).flatten()
    {
        
        draw_line_segment_mut(&mut approximated_image, line.0, line.1, black)
    }
    approximated_image.save("resources/approximated.png").unwrap();
}
