#![no_std]

pub trait System<T> {
    fn dxdt(&self, x: &[T], u: &[T], dxdt: &mut [T]);
    fn y(&self, x: &[T], u: &[T], y: &mut [T]);
    fn ndims(&self) -> (usize, usize, usize);
}
