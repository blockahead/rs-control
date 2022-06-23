pub trait System<T> {
    fn dxdt(&self, x: &[T], u: &[T]) -> Vec<T>;
    fn y(&self, x: &[T], u: &[T]) -> Vec<T>;
    fn ndims(&self) -> (usize, usize, usize);
}
