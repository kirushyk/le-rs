use crate::le::tensor::Tensor;

pub trait Model {
    fn predict(&mut self, x: &Tensor) -> Tensor;
}
