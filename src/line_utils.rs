use image::{ImageBuffer, Luma};
use itertools::iproduct;
use num::{traits::Euclid, Float};
use num_traits::NumCast;

/// Function emulates a line with thickness with multiple lines with thickness 1
pub fn thicken_line<T>(start: &(T, T), stop: &(T, T), thickness: T) -> Vec<((T, T), (T, T))>
where
    T: Float + Euclid,
{
    let length = length(start, stop);
    if length == T::from(0).unwrap() {
        return Vec::new();
    }
    let direction = ((start.0 - stop.0) / length, (start.1 - stop.1) / length);

    let mut lines = Vec::new();
    if length < T::from(1).unwrap() {
        lines.push((
            (
                start.0
                    + length / T::from(2f32).unwrap() * direction.0
                    + thickness * direction.1 / T::from(2f32).unwrap(),
                start.1 + length / T::from(2f32).unwrap() * direction.1
                    - thickness * direction.0 / T::from(2f32).unwrap(),
            ),
            (
                start.0 + length / T::from(2f32).unwrap() * direction.0
                    - thickness * direction.1 / T::from(2f32).unwrap(),
                start.1
                    + length / T::from(2f32).unwrap() * direction.1
                    + thickness * direction.0 / T::from(2f32).unwrap(),
            ),
        ))
    }
    for i in (0..<u32 as NumCast>::from(length).unwrap())
        .into_iter()
        .map(|i| T::from(i).unwrap())
    {
        lines.push((
            (
                start.0
                    + i * direction.0
                    + (T::from(2).unwrap() * (i.rem_euclid(&T::from(2).unwrap()))
                        - T::from(1).unwrap())
                        * thickness
                        * direction.1
                        / T::from(2).unwrap(),
                start.1 + i * direction.1
                    - (T::from(2).unwrap() * (i.rem_euclid(&T::from(2).unwrap()))
                        - T::from(1).unwrap())
                        * thickness
                        * direction.0
                        / T::from(2).unwrap(),
            ),
            (
                start.0
                    + (i + T::from(1).unwrap()) * direction.0
                    + (T::from(2).unwrap()
                        * ((i + T::from(1).unwrap()).rem_euclid(&T::from(2).unwrap()))
                        - T::from(1).unwrap())
                        * thickness
                        * direction.1
                        / T::from(2).unwrap(),
                start.1 + (i + T::from(1).unwrap()) * direction.1
                    - (T::from(2).unwrap()
                        * ((i + T::from(1).unwrap()).rem_euclid(&T::from(2).unwrap()))
                        - T::from(1).unwrap())
                        * thickness
                        * direction.0
                        / T::from(2).unwrap(),
            ),
        ))
    }
    lines.push((lines[lines.len() - 1].1, *stop));
    lines.insert(0, (*start, lines[0].0));
    lines
}

/// Function splits a single line in multiple lines, that each have a length of max_length or lower
pub fn partition_line<T>(start: &(T, T), stop: &(T, T), max_length: T) -> Vec<((T, T), (T, T))>
where
    T: Float + Euclid,
{
    let length = length(start, stop);
    let direction = ((stop.0 - start.0) / length, (stop.1 - start.1) / length);

    let num_partitions = <u32 as NumCast>::from((length / max_length).ceil()).unwrap();
    let delta = length / T::from(num_partitions).unwrap();
    (0..num_partitions)
        .into_iter()
        .map(|i| T::from(i).unwrap())
        .map(|i| {
            (
                (
                    start.0 + i * delta * direction.0,
                    start.1 + i * delta * direction.1,
                ),
                (
                    start.0 + (i + T::from(1).unwrap()) * delta * direction.0,
                    start.1 + (i + T::from(1).unwrap()) * delta * direction.1,
                ),
            )
        })
        .collect()
}

/// Function returning length of line
pub fn length<T>(start: &(T, T), stop: &(T, T)) -> T
where
    T: Float + Euclid,
{
    ((start.0 - stop.0).powf(T::from(2).unwrap()) + (start.1 - stop.1).powf(T::from(2).unwrap()))
        .sqrt()
}
