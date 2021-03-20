use crate::le::tensor::Tensor;
use crate::le::model::Model;
    
struct SVM {

}

#[allow(dead_code)]
struct SVMTrainingOptions {
    kernel: f32,
    c: f32
}

#[allow(dead_code)]
impl SVM {
    pub fn new() -> SVM {
        SVM {}
    }
    pub fn train(&mut self, _x: &Tensor, _y: &Tensor, _options: SVMTrainingOptions) {

    }
}

impl Model for SVM {
    fn predict(&mut self, _x: &Tensor) -> Tensor {
        Tensor::new_2d(&[&[1.0]])
    }
}

