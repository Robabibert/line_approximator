use image::{ImageBuffer, Luma};
use itertools::iproduct;
use num::{traits::Euclid, Float};
use num_traits::NumCast;

/// Function to read the brightness at given position interpolating to floating point positions
pub fn get_brightness<T>(image: &ImageBuffer<Luma<u8>, Vec<u8>>, x: T, y: T) -> T
where
    T: Float + Euclid,
{
    let x_factor = x.rem_euclid(&T::from(1).unwrap());
    let y_factor = y.rem_euclid(&T::from(1).unwrap());

    let x_factor_opp = T::from(1).unwrap() - x_factor;
    let y_factor_opp = T::from(1).unwrap() - y_factor;

    let x = <u32 as NumCast>::from(x).unwrap();
    let y = <u32 as NumCast>::from(y).unwrap();
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
