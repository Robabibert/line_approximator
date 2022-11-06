use image::{ImageBuffer, Luma};

use crate::{hilbert_curve, line_utils::{crop_to_scale, Length, partition_line, thicken_line, thicken_line_sin, thicken_lines_sin}, approximate};

pub fn approximate_image(image:&ImageBuffer<Luma<u8>,Vec<u8>>,order:usize,omega:f32)->Vec<((f32, f32), (f32, f32))>{

    let mut lines: Vec<((f32, f32), (f32, f32))> =
        hilbert_curve::HilbertCurve::new(order)
            .into_iter()
            .collect();
    crop_to_scale(&mut lines, image.width() as usize, image.height() as usize);
    let total_length = lines.get_total_length();
    let max_thickness = (image.width() * image.height()) as f32 / total_length;
    lines= lines
        .iter()
        .map(|line| partition_line(&line.0, &line.1, 1f32))
        .flatten()
        .collect();
    let thicknesses=lines.iter().map(|line|{
        approximate(&image, &line.0, &line.1, max_thickness)
    }).collect();
    return thicken_lines_sin(&lines, &thicknesses, omega);
    
}