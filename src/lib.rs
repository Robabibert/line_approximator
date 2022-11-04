use image::{ImageBuffer, Luma};
use itertools::iproduct;
use num::{traits::Euclid, Float};

/// Function returning length of line
fn length<T>(start: &(T, T), stop: &(T, T)) -> T
where
    T: Float + Euclid,
    u32: From<T>,
{
    ((start.0 - stop.0).powf(T::from(2).unwrap()) + (start.1 - stop.1).powf(T::from(2).unwrap()))
        .sqrt()
}

/// Function to read the brightness at given position interpolating to floating point positions
fn read_brightness<T>(image: &ImageBuffer<Luma<u8>, Vec<u8>>, x: T, y: T) -> T
where
    T: Float + Euclid,
    u32: From<T>,
{
    let x_factor = x.rem_euclid(&T::from(1).unwrap());
    let y_factor = y.rem_euclid(&T::from(1).unwrap());

    let x_factor_opp = T::from(1) - x_factor;
    let y_factor_opp = T::from(1) - y_factor;

    let x = u32::from(x);
    let y = u32::from(y);
    let width = image.width();
    let height = image.height();

    let on_site = T::from(
        image
            .get_pixel((x).min(width - 1).max(0), (y).min(height - 1).max(0))
            .0[0],
    )
    .unwrap()
        * (x_factor)
        * (y_factor);
    let dx = T::from(
        image
            .get_pixel((x + 1).min(width - 1).max(0), (y).min(height - 1).max(0))
            .0[0],
    )
    .unwrap()
        * (x_factor_opp)
        * (y_factor);
    let dy = T::from(
        image
            .get_pixel((x).min(width - 1).max(0), (y + 1).min(height - 1).max(0))
            .0[0],
    )
    .unwrap()
        * (x_factor)
        * (y_factor_opp);
    let dx_dy = T::from(
        image
            .get_pixel(
                (x + 1).min(width - 1).max(0),
                (y + 1).min(height - 1).max(0),
            )
            .0[0],
    )
    .unwrap()
        * (x_factor_opp)
        * (y_factor_opp);

    on_site + dx + dy + dx_dy
}
///Function that returns the thickness such that a line best approximates the brightness in its region
pub fn approximate<T>(
    image: &ImageBuffer<Luma<u8>, Vec<u8>>,
    start: &(T, T),
    stop: &(T, T),
    max_thickness: T,
) -> T
where
    T: Float + Euclid + std::iter::Sum,
    u32: From<T>,
{
    //length of line
    let length = length(start, stop);
    
    // number of steps to sample line (num) and thickness (num_perp)
    let num = u32::from(length).min(1);
    let num_perp = u32::from(max_thickness).min(1);
    let num_perp_T_half = T::from(num_perp).unwrap()*T::from(0.5).unwrap();

    // stepping size in direction of line (delta) and perpendicular to line (delta_perp)
    let delta = T::from(1).unwrap() / length;
    let delta_perp = T::from(1).unwrap() / max_thickness;

    //line direction and direction perpendicular to line
    let direction = ((stop.0 - start.0) / length, (stop.1 - start.1) / length);
    let direction_perp = (direction.1,-direction.0);

    let mean_brightness = iproduct!((0..num),(0..num_perp))
        .into_iter()
        .map(|(i,i_perp)| (T::from(i).unwrap(),T::from(i_perp).unwrap()))
        .map(|(i,i_perp)| {
            // sample pixels that fall within the line, respecting the thickness of the line
            let x = start.0 + i * delta * direction.0+(i_perp-num_perp_T_half)*delta_perp*direction_perp.0;
            let y = start.1 + i * delta * direction.1+(i_perp-num_perp_T_half)*delta_perp*direction_perp.1;
            (x, y)
        })
        .map(|(x, y)| read_brightness(image, x, y))
        .sum::<T>()
        / T::from(num*num_perp).unwrap();
    // The line is assumed to be black -> if mean_brightness==255 line thickness should be 0. if mean_brightness==0 line thickness should be max_thickness

    (T::from(255).unwrap()-mean_brightness)*max_thickness

}
