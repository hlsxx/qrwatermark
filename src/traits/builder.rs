pub trait Builder<T> {
  fn new() -> Self;
  fn build(self) -> T;
}
