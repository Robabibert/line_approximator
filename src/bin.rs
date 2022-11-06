use clap::{Arg, Parser};
use htree::HTree;
use image::{ImageBuffer, Luma};
use imageproc::drawing::draw_line_segment_mut;
use line_approximator_lib::{
    approximate,
    line_utils::{crop_to_scale, thicken_line, Length, partition_line}, hilbert_curve,
};
use std::env;

/// Program to approximate image with an HTree
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of image input fname
    input: String,

    /// Number of fractal order
    order: u8,

    output: String,
}

pub fn main() {
    let args = Args::parse();
    let image = match image::open(args.input) {
        Ok(image) => image.to_luma8(),
        Err(err) => panic!("File could not be opened {:?}", err),
    };
    //let mut lines: Vec<((f32, f32), (f32, f32))> =
    //    HTree::new(args.order as usize).into_iter().collect();

    let mut lines: Vec<((f32, f32), (f32, f32))> =
        hilbert_curve::HilbertCurve::new(args.order as usize).into_iter().collect();
    crop_to_scale(&mut lines, image.width() as usize, image.height() as usize);
    let total_length = lines.get_total_length();
    let max_thickness = (image.width() * image.height()) as f32 / total_length;

    let mut approximated_image: ImageBuffer<Luma<u8>, Vec<u8>> = ImageBuffer::new(image.width(), image.height());
    approximated_image.fill(255u8);
    let black = Luma([0u8]);


    for (start, stop) in lines
        .iter()
        .map(|line|partition_line(&line.0, &line.1, 1f32))
        .flatten()
        .map(|(start, stop)| {
            let thickness = approximate(&image, &start, &stop, max_thickness);
            thicken_line(&start, &stop, thickness)
        })
        .flatten()
    {
        draw_line_segment_mut(&mut approximated_image, start, stop, black)
    }

    approximated_image.save(args.output).unwrap();
}
