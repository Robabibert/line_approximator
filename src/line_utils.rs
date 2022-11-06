use std::f64::consts::PI;

use image::{ImageBuffer, Luma};
use itertools::iproduct;
use num::{traits::Euclid, Float, Num};
use num_traits::NumCast;

pub trait Length<T>
where
    T: Float + Euclid + std::iter::Sum,
{
    fn get_total_length(&self) -> T;
}

impl<T> Length<T> for Vec<((T, T), (T, T))>
where
    T: Float + Euclid + std::iter::Sum,
{
    fn get_total_length(&self) -> T {
        let two = T::from(2).unwrap();
        self.iter()
            .map(|(start, stop)| {
                ((start.0 - stop.0).powf(two) + (start.1 - stop.1).powf(two)).sqrt()
            })
            .sum::<T>()
    }
}

pub fn set_scale<T>(lines: &mut Vec<((T, T), (T, T))>, width: usize, height: usize)
where
    T: Float + Euclid,
{
    let min_x = lines
        .iter()
        .map(|(start, stop)| vec![start.0, stop.0])
        .flatten()
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let max_x = lines
        .iter()
        .map(|(start, stop)| vec![start.0, stop.0])
        .flatten()
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let min_y = lines
        .iter()
        .map(|(start, stop)| vec![start.1, stop.1])
        .flatten()
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();
    let max_y = lines
        .iter()
        .map(|(start, stop)| vec![start.1, stop.1])
        .flatten()
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap();

    let width = T::from(width).unwrap();
    let height = T::from(height).unwrap();
    for (start, stop) in lines.iter_mut() {
        start.0 = (start.0 - min_x) * width / (max_x - min_x);
        stop.0 = (stop.0 - min_x) * width / (max_x - min_x);
        start.1 = (start.1 - min_y) * height / (max_y - min_y);
        stop.1 = (stop.1 - min_y) * height / (max_y - min_y);
    }
}

pub fn crop_to_scale<T>(lines: &mut Vec<((T, T), (T, T))>, width: usize, height: usize)
where
    T: Float + Euclid,
{
    set_scale(lines, width.max(height), width.max(height));
    let width = T::from(width).unwrap();
    let height = T::from(height).unwrap();
    lines.retain(|(start, stop)| {
        if start.0 > width {
            return false;
        }
        if stop.0 > width {
            return false;
        }
        if start.1 > height {
            return false;
        }
        if stop.1 > height {
            return false;
        }
        true
    });
}

/// Function emulates a line with thickness with multiple lines with thickness 1
pub fn thicken_line_sin<T>(
    start: &(T, T),
    stop: &(T, T),
    thickness: T,
    periods: Option<usize>,
) -> Vec<((T, T), (T, T))>
where
    T: Float + Euclid,
{
    let length = length(start, stop);
    let periods = match periods {
        Some(periods) => T::from(periods).unwrap(),
        None => length.ceil(),
    };

    if length == T::from(0).unwrap() {
        return Vec::new();
    }
    let direction = ((start.0 - stop.0) / length, (start.1 - stop.1) / length);
    let num = periods * T::from(10).unwrap();

    let omega = T::from(2f64 * PI).unwrap() * periods;
    let a = thickness / T::from(2).unwrap();
    (0..<u32 as NumCast>::from(num).unwrap())
        .into_iter()
        .map(|i| (i, i + 1))
        .map(|(i, j)| (T::from(i).unwrap() / num, T::from(j).unwrap() / num))
        .map(|(t, t_next)| {
            let start = (start.0 + t * direction.0, start.1 + t * direction.1);
            let start = (
                start.0 - direction.1 * (t * omega).sin() * a,
                start.1 + direction.0 * (t * omega).sin() * a,
            );
            let stop = (
                start.0 + t_next * direction.0,
                start.1 + t_next * direction.1,
            );
            let stop = (
                stop.0 - direction.1 * (t_next * omega).sin() * a,
                stop.1 + direction.0 * (t_next * omega).sin() * a,
            );
            (start, stop)
        })
        .collect()
}

fn get_direction<T>(line: &((T, T), (T, T))) -> (T, T)
where
    T: Float + Euclid + std::ops::AddAssign + std::iter::Sum<T>,
{
    let length = length(&line.0, &line.1);
    (
        (line.0 .0 - line.1 .0) / length,
        (line.0 .1 - line.1 .1) / length,
    )
}

fn get_angle<T>(line1: &((T, T), (T, T)), line2: &((T, T), (T, T))) -> T
where
    T: Float + Euclid + std::ops::AddAssign + std::iter::Sum<T>,
{
    let direction1 = get_direction(line1);
    let direction2 = get_direction(line2);
    let angle = (direction1.0 * direction2.0 + direction1.1 * direction2.1).acos();
    angle
}

pub fn smooth_corners<T>(lines: &Vec<((T, T), (T, T))>)->Vec<((T, T), (T, T))>
where
    T: Float + Euclid + std::ops::AddAssign + std::iter::Sum<T>,
{
    let mut lines=lines.clone();
    let mut resulting_lines:Vec<((T, T),(T, T))>=Vec::new();
    let mut hit=false;
    let one=T::from(1).unwrap();
    let two=T::from(2).unwrap();
    for i in 0..lines.len() - 1 {
        let line1 = lines[i];
        let line2 = lines[i + 1];
        if line1.1 != line2.0 {
            continue;
        }
        let direction1=get_direction(&line1);
        let direction2=get_direction(&line2);
        
        if direction1.0!=direction2.0 ||direction1.1!=direction2.1{
            hit=true;
            let control_point=line1.1;
            let start:(T,T)=(
                (line1.0.0+line1.1.0)/two,
                (line1.0.1+line1.1.1)/two
            );
            let stop:(T,T)=(
                (line2.0.0+line2.1.0)/two,
                (line2.0.1+line2.1.1)/two
            );
            resulting_lines.push((line1.0,start));
            let num=2.max(<u32 as NumCast>::from(length(&line1.0, &line1.1)+length(&line2.0, &line2.1)).unwrap()/2);
            for (t,t_next) in (0..num).into_iter().map(|i|(T::from(i).unwrap()/T::from(num).unwrap(),T::from(i+1).unwrap()/T::from(num).unwrap())){
                resulting_lines.push(
                    ((
                        (one-t).powf(two)*start.0 + two*(one-t)*t*control_point.0+t.powf(two)*stop.0,
                        (one-t).powf(two)*start.1 + two*(one-t)*t*control_point.1+t.powf(two)*stop.1
                    ),
                    (
                        (one-t_next).powf(two)*start.0 + two*(one-t_next)*t_next*control_point.0+t_next.powf(two)*stop.0,
                        (one-t_next).powf(two)*start.1 + two*(one-t_next)*t_next*control_point.1+t_next.powf(two)*stop.1
                    ))
                )
            }
            //resulting_lines.push((stop,line2.1));
            lines[i+1]=(stop,line2.1);

        }else{
            hit=false;
            resulting_lines.push(line1);
        }
    }
    if !hit{
        resulting_lines.push(*lines.iter().last().unwrap());
    }
    resulting_lines
}
/// Function emulates a line with thickness with multiple lines with thickness 1
pub fn thicken_lines_sin<T>(
    lines: &Vec<((T, T), (T, T))>,
    thicknesses: &Vec<T>,
    omega: T,
) -> Vec<((T, T), (T, T))>
where
    T: Float + Euclid + std::ops::AddAssign + std::iter::Sum<T>,
{
    //omega:
    let mut total_length = T::from(0).unwrap();
    lines
        .iter()
        .zip(thicknesses)
        .map(|((start, stop), thickness)| {
            if *thickness<T::from(1).unwrap(){
                return Vec::new();
            }
            let segment_length = length(start, stop);
            let direction = (
                (start.0 - stop.0) / segment_length,
                (start.1 - stop.1) / segment_length,
            );

            let num = 2.max(<u32 as NumCast>::from(segment_length).unwrap());
            let points: Vec<(T, T)> = (0..num)
                .into_iter()
                .map(|i| {
                    let s = T::from(i).unwrap() / T::from(num).unwrap(); // s in [0,1)
                    let t = total_length + s * segment_length;
                    let sin_offset = (
                        -direction.1 * (*thickness) * (t * omega).sin(),
                        direction.0 * (*thickness) * (t * omega).sin(),
                    );
                    let point = (start.0 + s * direction.0, start.1 + s * direction.1);
                    (point.0 + sin_offset.0, point.1 + sin_offset.1)
                })
                .collect();

            total_length += segment_length;
            points
                .iter()
                .zip(points.iter().skip(1))
                .map(|(start, stop)| (*start, *stop))
                .collect::<Vec<((T, T), (T, T))>>()
        })
        .flatten()
        .collect()
}

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
