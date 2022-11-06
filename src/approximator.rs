use image::{ImageBuffer, Luma};

use crate::{hilbert_curve, line_utils::{crop_to_scale, Length, partition_line, thicken_line, thicken_line_sin, thicken_lines_sin,  smooth_corners}, approximate};

pub fn approximate_image(image:&ImageBuffer<Luma<u8>,Vec<u8>>,order:usize,omega:f32)->Vec<((f32, f32), (f32, f32))>{
    let mut image=image.clone();
    let min=*image.iter().min().unwrap();
    let max=*image.iter().max().unwrap();
    for pixel in image.iter_mut(){
        // heighten contrast
        *pixel=(((*pixel-min) as f32/(max-min) as f32)*255f32) as u8;
    }
    let mut lines: Vec<((f32, f32), (f32, f32))> =
        hilbert_curve::HilbertCurve::new(order)
            .into_iter()
            .collect();
    crop_to_scale(&mut lines, image.width() as usize, image.height() as usize);
    lines=smooth_corners(&lines);
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