use clap::{Arg, Parser};
use htree::HTree;
use image::{ImageBuffer, Luma};
use imageproc::drawing::draw_line_segment_mut;
use line_approximator_lib::{
    approximate, hilbert_curve,
    line_utils::{crop_to_scale, partition_line, thicken_line, Length}, approximator::approximate_image,
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
    
    let mut approximated_image: ImageBuffer<Luma<u8>, Vec<u8>> =
        ImageBuffer::new(image.width(), image.height());
    approximated_image.fill(255u8);
    let black = Luma([0u8]);

    let lines =approximate_image(&image, args.order as usize,1f32);
    for (start, stop) in lines {
        draw_line_segment_mut(&mut approximated_image, start, stop, black)
    }
    approximated_image.save(args.output).unwrap();
}
