use nalgebra::DVector;

pub trait System<T> {
    fn dxdt(&self, x: &DVector<T>, u: &DVector<T>) -> DVector<T>;
    fn y(&self, x: &DVector<T>, u: &DVector<T>) -> DVector<T>;
    fn ndims(&self) -> (usize, usize, usize);
}
