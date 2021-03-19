use crate::le::tensor::Tensor;
use crate::le::model::Model;
    
struct SVM {

}

struct SVMTrainingOptions {
    kernel: f32,
    c: f32
}

impl SVM {
    pub fn new() -> SVM {
        SVM {}
    }
    pub fn train(&mut self, x: &Tensor, y: &Tensor, options: SVMTrainingOptions) {

    }
}

impl Model for SVM {
    fn predict(&mut self, x: &Tensor) -> Tensor {
        Tensor::new()
    }
}

