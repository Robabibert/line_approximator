
pub mod line_utils;
mod image_utils;
pub mod hilbert_curve;
use image::{ImageBuffer, Luma};
use itertools::iproduct;
use crate::line_utils::{length,partition_line};
use crate::image_utils::{get_brightness};
use num::{traits::Euclid, Float};
use num_traits::NumCast;

///Function that returns the thickness such that a line best approximates the brightness in its region
pub fn approximate<T>(
    image: &ImageBuffer<Luma<u8>, Vec<u8>>,
    start: &(T, T),
    stop: &(T, T),
    max_thickness: T,
) -> T
where
    T: Float + Euclid + std::iter::Sum
{
    //length of line
    let length = length(start, stop);

    // number of steps to sample line (num) and thickness (num_perp)
    let num = <u32 as NumCast>::from(length).unwrap().max(1);
    let num_perp = <u32 as NumCast>::from(max_thickness).unwrap().max(1);
    let num_perp_half = T::from(num_perp).unwrap() * T::from(0.5).unwrap();

    // stepping size in direction of line (delta) and perpendicular to line (delta_perp)
    let delta = T::from(1).unwrap() / length;
    let delta_perp = T::from(1).unwrap() / max_thickness;

    //line direction and direction perpendicular to line
    let direction = ((stop.0 - start.0) / length, (stop.1 - start.1) / length);
    let direction_perp = (direction.1, -direction.0);

    let mean_brightness = iproduct!((0..num), (0..num_perp))
        .into_iter()
        .map(|(i, i_perp)| (T::from(i).unwrap(), T::from(i_perp).unwrap()))
        .map(|(i, i_perp)| {
            // sample pixels that fall within the line, respecting the thickness of the line
            let x = start.0
                + i * delta * direction.0
                + (i_perp - num_perp_half) * delta_perp * direction_perp.0;
            let y = start.1
                + i * delta * direction.1
                + (i_perp - num_perp_half) * delta_perp * direction_perp.1;
            (x, y)
        })
        .map(|(x, y)| get_brightness(image, x, y))
        .sum::<T>()
        / T::from(num * num_perp).unwrap();
    // The line is assumed to be black -> if mean_brightness==255 line thickness should be 0. if mean_brightness==0 line thickness should be max_thickness

    (T::from(255).unwrap() - mean_brightness)/T::from(255).unwrap() * max_thickness
}
