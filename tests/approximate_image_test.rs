use htree::HTree;
use image::{self, ImageBuffer, Luma, DynamicImage};
use imageproc::{drawing::{draw_line_segment_mut, draw_filled_rect_mut, draw_antialiased_line_segment}, rect::Rect};
use line_approximator::{
    approximate,
    line_utils::{length, partition_line, thicken_line},
};

#[test]
fn approximate_image() {
    let target_image:ImageBuffer<Luma<u8>, Vec<u8>> = image::open("resources/hawaii.jpg").unwrap().to_luma8();
    
    let mut approximated_image: ImageBuffer<Luma<u8>, Vec<u8>> = ImageBuffer::new(target_image.width(), target_image.height());
    // white background
    approximated_image.fill(255u8);

    
    let max_thickness = 20f32;
    let lines:Vec<((f32,f32),(f32,f32))>=(0..target_image.height()).step_by(max_thickness as usize).into_iter().map(|y|{
        let start=(0f32,y as f32);
        let stop=(target_image.width() as f32,y as f32);
        (start,stop)
    }).collect();
    
    

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
