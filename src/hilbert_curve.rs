use hilbert_index::FromHilbertIndex;

use num::Float;
use std::marker::PhantomData;


#[derive(Clone, Copy, Debug)]
pub struct HilbertCurve<T> {
    order: usize,
    _marker: PhantomData<T>,
}

pub struct HilbertCurveIterator<T>
where
    T: Float,
{
    hilbert_curve: HilbertCurve<T>,
    index: usize,
}


impl<T> HilbertCurve<T>
where
    T: Float,
{
    pub fn new(order: usize) -> HilbertCurve<T> {
        HilbertCurve {
            order,
            _marker: PhantomData {},
        }
    }
}
impl<T> Iterator for HilbertCurveIterator<T> where T:Float {
    type Item = ((T, T), (T, T));

    fn next(&mut self) -> Option<Self::Item> {
        if self.index as i32-1 >= (2u32.pow(2u32 * self.hilbert_curve.order as u32)) as i32 {
            return None;
        }
        let position_start: [usize; 2] =
            self.index.from_hilbert_index(self.hilbert_curve.order.into());
        let position_end: [usize; 2] =
            (self.index + 1).from_hilbert_index(self.hilbert_curve.order.into());
        self.index += 1;
        Some((
            (
                T::from(position_start[0]).unwrap() / T::from(2u32.pow(self.hilbert_curve.order as u32)).unwrap(),
                T::from(position_start[1]).unwrap() / T::from(2u32.pow(self.hilbert_curve.order as u32)).unwrap()
            ),
            (
                T::from(position_end[0]).unwrap() / T::from(2u32.pow(self.hilbert_curve.order as u32)).unwrap(),
                T::from(position_end[1]).unwrap() / T::from(2u32.pow(self.hilbert_curve.order as u32)).unwrap()
            ),
        ))
    }
}

impl<T> IntoIterator for HilbertCurve<T>
where
    T: Float,
{
    type Item = ((T, T), (T, T));
    type IntoIter = HilbertCurveIterator<T>;


    fn into_iter(self) -> Self::IntoIter {
        HilbertCurveIterator {
            hilbert_curve: self,
            index: 0,
        }
    }
}
